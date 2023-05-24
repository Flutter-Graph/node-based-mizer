pub use rename_ports::*;
pub use rework_layout_controls_to_not_use_nodes::*;
pub use rework_midi_config::*;

mod rename_ports;
mod rework_layout_controls_to_not_use_nodes;
mod rework_midi_config;

pub trait ProjectFileMigration: Clone + Copy {
    const VERSION: usize;

    fn migrate(&self, project_file: &mut String) -> anyhow::Result<()>;
}