use std::sync::Arc;

use crate::datastore::Datastore;

pub struct ContextData {
    pub datastore: Arc<Datastore>,
}

impl Default for ContextData {
    fn default() -> Self {
        Self {
            datastore: Default::default(),
        }
    }
}
