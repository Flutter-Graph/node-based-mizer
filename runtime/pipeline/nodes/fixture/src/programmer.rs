use serde::{Deserialize, Serialize};
use mizer_fixtures::definition::{ColorChannel, FixtureFaderControl};
use mizer_fixtures::manager::FixtureManager;
use mizer_fixtures::programmer::Programmer;

use mizer_node::*;

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProgrammerNode;

impl PipelineNode for ProgrammerNode {
    fn details(&self) -> NodeDetails {
        NodeDetails {
            name: "ProgrammerNode".into(),
            preview_type: PreviewType::None,
        }
    }

    fn list_ports(&self) -> Vec<(PortId, PortMetadata)> {
        vec![
            ("Intensity".into(), PortMetadata {
                port_type: PortType::Single,
                direction: PortDirection::Input,
                ..Default::default()
            }),
            ("Shutter".into(), PortMetadata {
                port_type: PortType::Single,
                direction: PortDirection::Input,
                ..Default::default()
            }),
            ("Color".into(), PortMetadata {
                port_type: PortType::Color,
                direction: PortDirection::Input,
                ..Default::default()
            }),
            ("Pan".into(), PortMetadata {
                port_type: PortType::Single,
                direction: PortDirection::Input,
                ..Default::default()
            }),
            ("Tilt".into(), PortMetadata {
                port_type: PortType::Single,
                direction: PortDirection::Input,
                ..Default::default()
            }),
            ("Focus".into(), PortMetadata {
                port_type: PortType::Single,
                direction: PortDirection::Input,
                ..Default::default()
            }),
            ("Zoom".into(), PortMetadata {
                port_type: PortType::Single,
                direction: PortDirection::Input,
                ..Default::default()
            }),
            ("Prism".into(), PortMetadata {
                port_type: PortType::Single,
                direction: PortDirection::Input,
                ..Default::default()
            }),
            ("Iris".into(), PortMetadata {
                port_type: PortType::Single,
                direction: PortDirection::Input,
                ..Default::default()
            }),
            ("Frost".into(), PortMetadata {
                port_type: PortType::Single,
                direction: PortDirection::Input,
                ..Default::default()
            }),
        ]
    }

    fn node_type(&self) -> NodeType {
        NodeType::Programmer
    }
}

impl ProcessingNode for ProgrammerNode {
    type State = ();

    fn process(&self, context: &impl NodeContext, state: &mut Self::State) -> anyhow::Result<()> {
        if let Some(fixture_manager) = context.inject::<FixtureManager>() {
            let mut programmer = fixture_manager.get_programmer();
            if let Some(intensity) = context.read_port::<_, f64>("Intensity") {
                programmer.write_control(FixtureFaderControl::Intensity, intensity);
            }
            if let Some(shutter) = context.read_port::<_, f64>("Shutter") {
                programmer.write_control(FixtureFaderControl::Shutter, shutter);
            }
            if let Some(color) = context.read_port::<_, Color>("Color") {
                programmer.write_control(FixtureFaderControl::Color(ColorChannel::Red), color.red);
                programmer.write_control(FixtureFaderControl::Color(ColorChannel::Green), color.green);
                programmer.write_control(FixtureFaderControl::Color(ColorChannel::Blue), color.blue);
            }
            if let Some(pan) = context.read_port::<_, f64>("Pan") {
                programmer.write_control(FixtureFaderControl::Pan, pan);
            }
            if let Some(tilt) = context.read_port::<_, f64>("Tilt") {
                programmer.write_control(FixtureFaderControl::Tilt, tilt);
            }
            if let Some(focus) = context.read_port::<_, f64>("Focus") {
                programmer.write_control(FixtureFaderControl::Focus, focus);
            }
            if let Some(zoom) = context.read_port::<_, f64>("Zoom") {
                programmer.write_control(FixtureFaderControl::Zoom, zoom);
            }
            if let Some(prism) = context.read_port::<_, f64>("Prism") {
                programmer.write_control(FixtureFaderControl::Prism, prism);
            }
            if let Some(iris) = context.read_port::<_, f64>("Iris") {
                programmer.write_control(FixtureFaderControl::Iris, iris);
            }
            if let Some(frost) = context.read_port::<_, f64>("Frost") {
                programmer.write_control(FixtureFaderControl::Frost, frost);
            }
        }

        Ok(())
    }

    fn create_state(&self) -> Self::State {
        Default::default()
    }
}
