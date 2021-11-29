use mizer_protocol_dmx::{DmxConnectionManager, SacnOutput, ArtnetOutput};

use crate::{Project, ProjectManagerMut, ConnectionTypes, ConnectionConfig};

impl ProjectManagerMut for DmxConnectionManager {
    fn new(&mut self) {
        self.add_output("dmx-0".into(), SacnOutput::new());
    }

    fn load(&mut self, project: &Project) -> anyhow::Result<()> {
        for connection in &project.connections {
            match &connection.config {
                ConnectionTypes::Sacn => self.add_output(connection.id.clone(), SacnOutput::new()),
                ConnectionTypes::Artnet { port, host } => self.add_output(connection.id.clone(), ArtnetOutput::new(host.clone(), *port)?)
            }
        }

        Ok(())
    }

    fn save(&self, project: &mut Project) {
        for (id, output) in self.list_outputs() {
            project.connections.push(ConnectionConfig {
                id: id.clone(),
                name: output.name(),
                config: ConnectionTypes::Sacn,
            });
        }
    }

    fn clear(&mut self) {
        self.clear();
    }
}