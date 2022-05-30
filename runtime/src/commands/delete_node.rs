use crate::pipeline_access::PipelineAccess;
use mizer_commander::{Command, Ref, RefMut};
use mizer_execution_planner::{ExecutionNode, ExecutionPlanner};
use mizer_layouts::{ControlConfig, LayoutStorage};
use mizer_node::{NodeDesigner, NodeLink, NodePath};
use mizer_nodes::Node;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
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
            let (matching, non_matching) = layout
                .controls
                .clone()
                .into_iter()
                .partition(|control| &control.node == &self.path);
            layout.controls = non_matching;
            controls.insert(layout.id.clone(), matching);
        }
        layout_storage.set(layouts);

        Ok(((), (node, designer, links, controls)))
    }

    fn revert(
        &self,
        (pipeline, planner, layout_storage): (
            &mut PipelineAccess,
            &mut ExecutionPlanner,
            &LayoutStorage,
        ),
        (node, designer, links, mut controls): Self::State,
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::DeleteNodeCommand;
    use crate::pipeline_access::PipelineAccess;
    use mizer_commander::Command;
    use mizer_execution_planner::ExecutionPlanner;
    use mizer_layouts::{
        ControlConfig, ControlDecorations, ControlPosition, ControlSize, Layout, LayoutStorage,
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
        pipeline_access.add_link(NodeLink {
            source: path1.clone(),
            source_port: "value".into(),
            target: path2,
            target_port: "value".into(),
            port_type: PortType::Single,
            local: true,
        });
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
                node: path.clone(),
                position: ControlPosition::default(),
                label: None,
                decoration: ControlDecorations::default(),
                size: ControlSize {
                    width: 1,
                    height: 1,
                },
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