use std::time::{SystemTime, UNIX_EPOCH};

use mizer_node_api::*;
use std::fmt::Formatter;

fn now() -> u128 {
    let time = SystemTime::now();
    let time = time
        .duration_since(UNIX_EPOCH)
        .expect("There shouldn't be a case where the unix epoch is after the current time");

    time.as_millis()
}

pub struct ClockNode {
    last_tick: u128,
    frame: f64,
    /// BPM
    speed: f64,
    outputs: Vec<ClockSender>,
}

impl ClockNode {
    pub fn new(speed: f64) -> Self {
        ClockNode {
            last_tick: 0,
            frame: 0f64,
            speed,
            outputs: Vec::new(),
        }
    }

    pub fn get_clock_channel(&mut self) -> ClockChannel {
        let (sender, channel) = ClockChannel::new();
        self.outputs.push(sender);

        channel
    }
}

impl Default for ClockNode {
    fn default() -> Self {
        ClockNode::new(90.)
    }
}

impl ProcessingNode for ClockNode {
    fn get_details(&self) -> NodeDetails {
        NodeDetails::new("ClockNode")
            .with_outputs(vec![NodeOutput::new("clock", NodeChannel::Clock)])
            .with_properties(vec![NodeProperty::numeric("speed")])
    }

    fn process(&mut self) {
        let tick = now();
        let delta: f64 = (tick - self.last_tick) as f64 * (self.speed / 60000f64);
        self.frame += delta;
        let downbeat = self.frame > 4f64;
        while self.frame > 4f64 {
            self.frame -= 4f64;
        }
        self.last_tick = tick;

        let clock_beat = ClockBeat { delta, downbeat };
        for output in &self.outputs {
            output.send(clock_beat);
        }
    }

    fn set_numeric_property(&mut self, property: &str, value: f64) {
        if property == "speed" {
            self.speed = value;
        }
    }
}

impl SourceNode for ClockNode {}

impl DestinationNode for ClockNode {
    fn connect_to_clock_input(
        &mut self,
        output: &str,
        node: &mut impl SourceNode,
        input: &str,
    ) -> ConnectionResult {
        if output == "clock" {
            let (tx, channel) = ClockChannel::new();
            node.connect_clock_input(input, channel)?;
            self.outputs.push(tx);
            Ok(())
        } else {
            Err(ConnectionError::InvalidOutput(output.to_string()))
        }
    }
}

impl std::fmt::Debug for ClockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClockNode")
            .field("speed", &self.speed)
            .finish()
    }
}