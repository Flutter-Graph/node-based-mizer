use serde::{Deserialize, Serialize};

use mizer_commander::{Command, RefMut};
use mizer_execution_planner::ExecutionPlanner;
use mizer_node::NodeLink;

use crate::pipeline_access::PipelineAccess;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddLinkCommand {
    pub link: NodeLink,
}

impl<'a> Command<'a> for AddLinkCommand {
    type Dependencies = (RefMut<PipelineAccess>, RefMut<ExecutionPlanner>);
    type State = Option<NodeLink>;
    type Result = ();

    fn label(&self) -> String {
        format!(
            "Add Link from '{}:{}' to '{}:{}'",
            self.link.source, self.link.source_port, self.link.target, self.link.target_port
        )
    }

    fn apply(
        &self,
        (pipeline, planner): (&mut PipelineAccess, &mut ExecutionPlanner),
    ) -> anyhow::Result<(Self::Result, Self::State)> {
        let links = pipeline.links.read();

        let mut state = None;
        let metadata = pipeline
            .try_get_input_port_metadata(&self.link.target, &self.link.target_port)
            .ok_or_else(|| anyhow::anyhow!("Target port does not exist"))?;
        if metadata.multiple != Some(true) {
            if let Some(existing_link) = links
                .into_iter()
                .find(|l| l.target_port == self.link.target_port && l.target == self.link.target)
            {
                pipeline.remove_link(&existing_link);
                planner.remove_link(&existing_link);
                state = Some(existing_link);
            };
        }
        pipeline.add_link(self.link.clone())?;
        planner.add_link(self.link.clone());

        Ok(((), state))
    }

    fn revert(
        &self,
        (pipeline, planner): (&mut PipelineAccess, &mut ExecutionPlanner),
        previous_link: Self::State,
    ) -> anyhow::Result<()> {
        planner.remove_link(&self.link);
        pipeline.remove_link(&self.link);
        if let Some(previous_link) = previous_link {
            pipeline.add_link(previous_link.clone())?;
            planner.add_link(previous_link);
        }

        Ok(())
    }
}
