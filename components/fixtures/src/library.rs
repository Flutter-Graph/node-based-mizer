use crate::definition::FixtureDefinition;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct FixtureLibrary {
    providers: Arc<Vec<Box<dyn FixtureLibraryProvider>>>,
}

impl FixtureLibrary {
    pub fn new(providers: Vec<Box<dyn FixtureLibraryProvider>>) -> Self {
        FixtureLibrary {
            providers: Arc::new(providers),
        }
    }

    pub fn get_definition(&self, id: &str) -> Option<FixtureDefinition> {
        self.providers
            .iter()
            .filter_map(|provider| provider.get_definition(id))
            .collect::<Vec<_>>()
            .first()
            .cloned()
    }

    pub fn list_definitions(&self) -> Vec<FixtureDefinition> {
        self.providers
            .iter()
            .flat_map(|provider| provider.list_definitions())
            .collect()
    }
}

pub trait FixtureLibraryProvider: Send + Sync {
    fn load(&mut self) -> anyhow::Result<()>;

    fn get_definition(&self, id: &str) -> Option<FixtureDefinition>;

    fn list_definitions(&self) -> Vec<FixtureDefinition>;
}
