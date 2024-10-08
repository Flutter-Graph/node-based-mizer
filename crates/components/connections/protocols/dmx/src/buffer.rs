use dashmap::{DashMap, DashSet};

#[derive(Default)]
pub struct DmxBuffer {
    buffers: DashMap<u16, [u8; 512]>,
    touched: DashSet<u16>,
}

impl DmxBuffer {
    pub fn write_single(&self, universe: u16, channel: u16, value: u8) {
        assert!(channel < 512, "DMX Channel is above 512");
        tracing::trace!("writing {}:{} => {}", universe, channel, value);
        let mut buffer = self.buffers.entry(universe).or_insert_with(|| [0; 512]);
        buffer[channel as usize] = value;
        self.touched.insert(universe);
    }

    pub fn write_bulk(&self, universe: u16, channel: u16, values: &[u8]) {
        assert!(channel < 512, "DMX Channel is above 512");
        assert!(
            channel as usize + values.len() <= 512,
            "DMX Channel is above 512"
        );
        let mut buffer = self.buffers.entry(universe).or_insert_with(|| [0; 512]);
        for (i, value) in values.iter().enumerate() {
            buffer[i + channel as usize - 1usize] = *value;
        }
        self.touched.insert(universe);
    }

    pub fn iter(&self) -> impl Iterator<Item = (u16, [u8; 512])> + '_ {
        self.buffers
            .iter()
            .map(|entry| (*entry.key(), *entry.value()))
    }

    pub fn universes(&self) -> impl Iterator<Item = u16> + '_ {
        self.buffers.iter().map(|entry| *entry.key())
    }

    pub(crate) fn pristine(&self) {
        self.touched.clear();
    }

    pub(crate) fn cleanup(&self) {
        profiling::scope!("DmxBuffer::cleanup");
        self.buffers.retain(|key, _| self.touched.contains(key));
    }
}
