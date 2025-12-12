use std::sync::Arc;

use crate::datastore::Datastore;

pub struct ContextData {
    pub datastore: Arc<Datastore>,
}

impl ContextData {
    pub fn new(datastore: Arc<Datastore>) -> Self {
        Self { datastore }
    }
}
