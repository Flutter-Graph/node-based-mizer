use crate::pipeline_access::PipelineAccess;
use mizer_commander::{Command, Ref, RefMut};
use mizer_execution_planner::{ExecutionNode, ExecutionPlanner};
use mizer_layouts::{ControlConfig, ControlType, LayoutStorage};
use mizer_node::{NodeDesigner, NodeLink, NodePath, NodeType};
use mizer_nodes::{ContainerNode, Node, NodeDowncast};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteNodeCommand {
    pub path: NodePath,
}

impl<'a> Command<'a> for DeleteNodeCommand {
    type Dependencies = (
        RefMut<PipelineAccess>,
        RefMut<ExecutionPlanner>,
        Ref<LayoutStorage>,
    );
    type State = (
        Node,
        NodeDesigner,
        Vec<NodeLink>,
        HashMap<String, Vec<ControlConfig>>,
        Vec<(NodePath, Node)>,
    );
    type Result = ();

    fn label(&self) -> String {
        format!("Delete Node {}", self.path)
    }

    fn apply(
        &self,
        (pipeline, planner, layout_storage): (
            &mut PipelineAccess,
            &mut ExecutionPlanner,
            &LayoutStorage,
        ),
    ) -> anyhow::Result<(Self::Result, Self::State)> {
        let (node, designer, links) = pipeline.delete_node(self.path.clone())?;
        for link in &links {
            planner.remove_link(link);
        }
        planner.remove_node(&self.path);

        let mut layouts = layout_storage.read();
        let mut controls = HashMap::new();
        for layout in &mut layouts {
            let (matching, non_matching) =
                layout.controls.clone().into_iter().partition(|control| {
                    matches!(&control.control_type, ControlType::Node { path } if path == &self.path)
                });
            layout.controls = non_matching;
            controls.insert(layout.id.clone(), matching);
        }
        layout_storage.set(layouts);
        let update_node_commands = self.remove_node_from_containers(pipeline)?;

        Ok(((), (node, designer, links, controls, update_node_commands)))
    }

    fn revert(
        &self,
        (pipeline, planner, layout_storage): (
            &mut PipelineAccess,
            &mut ExecutionPlanner,
            &LayoutStorage,
        ),
        (node, designer, links, mut controls, container_commands): Self::State,
    ) -> anyhow::Result<()> {
        pipeline.internal_add_node(self.path.clone(), node, designer);
        planner.add_node(ExecutionNode {
            path: self.path.clone(),
            attached_executor: None,
        });
        for link in links {
            pipeline.add_link(link.clone())?;
            planner.add_link(link);
        }
        let mut layouts = layout_storage.read();
        for layout in &mut layouts {
            let mut controls = controls.remove(&layout.id).unwrap_or_default();
            layout.controls.append(&mut controls);
        }
        layout_storage.set(layouts);
        for (path, container) in container_commands {
            pipeline.apply_node_config(&path, container)?;
        }

        Ok(())
    }
}

impl DeleteNodeCommand {
    fn remove_node_from_containers(
        &self,
        pipeline: &mut PipelineAccess,
    ) -> anyhow::Result<Vec<(NodePath, Node)>> {
        let mut update_node_commands = Vec::new();
        for (path, node) in pipeline
            .nodes
            .iter()
            .filter(|(_, node)| node.node_type() == NodeType::Container)
        {
            if let Some(mut container) = node.downcast_node::<ContainerNode>(NodeType::Container) {
                let removed_node = container.nodes.iter().position(|p| p == &self.path);
                if let Some(removed_node_index) = removed_node {
                    container.nodes.remove(removed_node_index);
                    update_node_commands.push((path.clone(), container));
                }
            }
        }

        update_node_commands
            .into_iter()
            .map(|(path, node)| {
                let previous = pipeline.apply_node_config(&path, node.into());

                previous.map(|previous| (path, previous))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::DeleteNodeCommand;
    use crate::pipeline_access::PipelineAccess;
    use mizer_commander::Command;
    use mizer_execution_planner::ExecutionPlanner;
    use mizer_layouts::{
        ControlConfig, ControlDecorations, ControlPosition, ControlSize, ControlType, Layout,
        LayoutStorage,
    };
    use mizer_node::*;
    use mizer_nodes::FaderNode;
    use mizer_ports::PortType;
    use pinboard::NonEmptyPinboard;

    #[test]
    fn delete_node_should_remove_the_connected_links() {
        let mut pipeline_access = PipelineAccess::new();
        let mut planner = ExecutionPlanner::new();
        let layout_storage = LayoutStorage::new(NonEmptyPinboard::new(Default::default()));
        let path1 = NodePath("/node1".into());
        let path2 = NodePath("/node2".into());
        pipeline_access.internal_add_node(
            path1.clone(),
            FaderNode::default().into(),
            Default::default(),
        );
        pipeline_access.internal_add_node(
            path2.clone(),
            FaderNode::default().into(),
            Default::default(),
        );
        let injector = Injector::default();
        let ports = pipeline_access.nodes[&path1].list_ports(&injector);
        pipeline_access.ports.insert(path1.clone(), ports);
        let ports = pipeline_access.nodes[&path2].list_ports(&injector);
        pipeline_access.ports.insert(path2.clone(), ports);
        pipeline_access
            .add_link(NodeLink {
                source: path1.clone(),
                source_port: "Output".into(),
                target: path2,
                target_port: "Input".into(),
                port_type: PortType::Single,
                local: true,
            })
            .unwrap();
        let cmd = DeleteNodeCommand { path: path1 };

        cmd.apply((&mut pipeline_access, &mut planner, &layout_storage))
            .unwrap();

        let links = pipeline_access.links.read();
        assert!(links.is_empty());
    }

    #[test]
    fn delete_node_should_remove_layout_controls() {
        let mut pipeline_access = PipelineAccess::new();
        let mut planner = ExecutionPlanner::new();
        let layout_storage = LayoutStorage::new(NonEmptyPinboard::new(Default::default()));
        let path = NodePath("/node".into());
        pipeline_access.internal_add_node(
            path.clone(),
            FaderNode::default().into(),
            Default::default(),
        );
        let mut layouts = layout_storage.read();
        layouts.push(Layout {
            id: "".into(),
            controls: vec![ControlConfig {
                id: Default::default(),
                control_type: ControlType::Node { path: path.clone() },
                position: ControlPosition::default(),
                label: None,
                decoration: ControlDecorations::default(),
                size: ControlSize {
                    width: 1,
                    height: 1,
                },
                behavior: Default::default(),
            }],
        });
        layout_storage.set(layouts);
        let cmd = DeleteNodeCommand { path };

        cmd.apply((&mut pipeline_access, &mut planner, &layout_storage))
            .unwrap();

        let layouts = layout_storage.read();
        assert!(layouts[0].controls.is_empty());
    }
}
