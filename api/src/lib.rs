mod channels;

pub use crate::channels::*;

#[derive(Debug, Clone)]
pub struct NodeDetails {
    name: String,
    inputs: Vec<NodeInput>,
    outputs: Vec<NodeOutput>,
    properties: Vec<NodeProperty>
}

impl NodeDetails {
    pub fn new<S: Into<String>>(name: S) -> Self {
        NodeDetails {
            name: name.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            properties: Vec::new()
        }
    }

    pub fn with_inputs(self, inputs: Vec<NodeInput>) -> Self {
        NodeDetails {
            name: self.name,
            outputs: self.outputs,
            properties: self.properties,
            inputs
        }
    }

    pub fn with_outputs(self, outputs: Vec<NodeOutput>) -> Self {
        NodeDetails {
            name: self.name,
            inputs: self.inputs,
            properties: self.properties,
            outputs
        }
    }

    pub fn with_properties(self, properties: Vec<NodeProperty>) -> Self {
        NodeDetails {
            name: self.name,
            outputs: self.outputs,
            inputs: self.inputs,
            properties,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NodeChannel {
    Dmx,
    Numeric,
    Trigger,
    Clock,
    Video,
    Color,
    Vector,
    Text,
    Midi,
    Timecode,
    Boolean,
    Select
}

#[derive(Debug, Clone)]
pub struct NodeInput {
    pub name: String,
    pub channel_type: NodeChannel
}

impl NodeInput {
    pub fn new<S: Into<String>>(name: S, channel_type: NodeChannel) -> Self {
        NodeInput {
            name: name.into(),
            channel_type
        }
    }

    pub fn dmx<S: Into<String>>(name: S) -> Self {
        NodeInput {
            name: name.into(),
            channel_type: NodeChannel::Dmx
        }
    }

    pub fn numeric<S: Into<String>>(name: S) -> Self {
        NodeInput {
            name: name.into(),
            channel_type: NodeChannel::Numeric
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeOutput {
    pub name: String,
    pub channel_type: NodeChannel
}

impl NodeOutput {
    pub fn new<S: Into<String>>(name: S, channel_type: NodeChannel) -> Self {
        NodeOutput {
            name: name.into(),
            channel_type
        }
    }

    pub fn dmx<S: Into<String>>(name: S) -> Self {
        NodeOutput {
            name: name.into(),
            channel_type: NodeChannel::Dmx
        }
    }

    pub fn numeric<S: Into<String>>(name: S) -> Self {
        NodeOutput {
            name: name.into(),
            channel_type: NodeChannel::Numeric
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeProperty {
    pub name: String,
    pub property_type: PropertyType
}

impl NodeProperty {
    pub fn new<S: Into<String>>(name: S, property_type: PropertyType) -> Self {
        NodeProperty {
            name: name.into(),
            property_type
        }
    }

    pub fn numeric<S: Into<String>>(name: S) -> Self {
        NodeProperty {
            name: name.into(),
            property_type: PropertyType::Numeric
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PropertyType {
    Numeric
}

pub trait ProcessingNode: InputNode + OutputNode {
    fn get_details(&self) -> NodeDetails;

    fn process(&mut self) {}

    fn set_numeric_property<S: Into<String>>(&mut self, property: S, value: f64) {}
}

pub trait InputNode {
    fn connect_dmx_input(&mut self, channels: &[DmxChannel]) {}
    fn connect_numeric_input(&mut self, channel: NumericChannel) {}
    fn connect_trigger_input(&mut self, channel: TriggerChannel) {}
    fn connect_clock_input(&mut self, channel: ClockChannel) {}
    fn connect_video_input(&mut self, source: &impl gstreamer::ElementExt) {}
}

pub trait OutputNode {
    fn connect_to_dmx_input(&mut self, input: &mut impl InputNode) {}
    fn connect_to_numeric_input(&mut self, input: &mut impl InputNode) {}
    fn connect_to_trigger_input(&mut self, input: &mut impl InputNode) {}
    fn connect_to_clock_input(&mut self, input: &mut impl InputNode) {}
    fn connect_to_video_input(&mut self, input: &mut impl InputNode) {}
}

mod deps {
    pub use crossbeam_channel::{Receiver, Sender, TryRecvError};
    pub use crossbeam_channel::unbounded as channel;
}
