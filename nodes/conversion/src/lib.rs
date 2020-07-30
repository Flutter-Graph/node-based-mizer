use mizer_node_api::*;

pub struct ConvertToDmxNode {
    channel: u8,
    universe: u16,
    outputs: Vec<DmxSender>,
    inputs: Vec<NumericChannel>,
}

impl ConvertToDmxNode {
    pub fn new(universe: Option<u16>, channel: Option<u8>) -> Self {
        let universe = universe.unwrap_or_default();
        let channel = channel.unwrap_or_default();
        log::trace!("New ConvertToDmxNode({}.{:0>3})", universe + 1, channel + 1);

        ConvertToDmxNode {
            universe,
            channel,
            outputs: Vec::new(),
            inputs: Vec::new(),
        }
    }
}

impl ProcessingNode for ConvertToDmxNode {
    fn get_details(&self) -> NodeDetails {
        NodeDetails::new("ConvertToDmxNode")
            .with_inputs(vec![NodeInput::numeric("numeric")])
            .with_outputs(vec![NodeOutput::dmx("dmx")])
    }

    fn process(&mut self) {
        let mut last = None;
        for channel in &self.inputs {
            match channel.recv() {
                Ok(Some(value)) => last = Some(value),
                Ok(None) => {},
                Err(err) => println!("{:?}", err),
            }
        }
        if let Some(value) = last {
            let bounded = (value as i64).min(255).max(0) as u8;
            for tx in &self.outputs {
                tx.send(bounded);
            }
        }
    }
}
impl InputNode for ConvertToDmxNode {
    fn connect_numeric_input(&mut self, channel: NumericChannel) {
        self.inputs.push(channel);
    }
}
impl OutputNode for ConvertToDmxNode {
    fn connect_to_dmx_input(&mut self, input: &mut impl InputNode) {
        let (tx, channel) = DmxChannel::new(self.universe, self.channel);
        self.outputs.push(tx);
        input.connect_dmx_input(&[channel]);
    }
}
