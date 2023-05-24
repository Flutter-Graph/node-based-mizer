use mizer_node::*;
use mizer_timecode::{TimecodeControlId, TimecodeManager};
use serde::{Deserialize, Serialize};

const VALUE_OUTPUT: &str = "Value";

const CONTROL_ID_SETTING: &str = "Control";

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct TimecodeOutputNode {
    pub control_id: TimecodeControlId,
}

impl ConfigurableNode for TimecodeOutputNode {
    fn settings(&self, injector: &Injector) -> Vec<NodeSetting> {
        let manager = injector.get::<TimecodeManager>().unwrap();
        let controls = manager
            .controls()
            .into_iter()
            .map(|control| IdVariant {
                value: control.id.into(),
                label: control.name,
            })
            .collect();

        vec![setting!(id CONTROL_ID_SETTING, self.control_id, controls)]
    }

    fn update_setting(&mut self, setting: NodeSetting) -> anyhow::Result<()> {
        update!(id setting, CONTROL_ID_SETTING, self.control_id);

        update_fallback!(setting)
    }
}

impl PipelineNode for TimecodeOutputNode {
    fn details(&self) -> NodeDetails {
        NodeDetails {
            name: stringify!(TimecodeOutputNode).into(),
            preview_type: PreviewType::History,
        }
    }

    fn list_ports(&self) -> Vec<(PortId, PortMetadata)> {
        vec![output_port!(VALUE_OUTPUT, PortType::Single)]
    }

    fn node_type(&self) -> NodeType {
        NodeType::TimecodeOutput
    }
}

impl ProcessingNode for TimecodeOutputNode {
    type State = ();

    fn process(&self, context: &impl NodeContext, _: &mut Self::State) -> anyhow::Result<()> {
        let timecode = context
            .inject::<TimecodeManager>()
            .ok_or_else(|| anyhow::anyhow!("Missing Timecode Module"))?;
        if let Some(value) = timecode.get_control_value(self.control_id) {
            context.write_port(VALUE_OUTPUT, value);
            context.push_history_value(value);
        }
        Ok(())
    }

    fn create_state(&self) -> Self::State {
        Default::default()
    }
}