use std::sync::Arc;

use dashmap::{mapref::one::Ref, DashMap};

use super::Session;

#[derive(Default)]
pub struct SessionManager {
    session_map: DashMap<u64, Arc<Session>>,
}

impl SessionManager {
    pub fn add(&self, session: Arc<Session>) {
        self.session_map
            .insert(session.get_global_session_id(), session);
    }

    pub fn get(&self, gateway_id: u32, session_id: u32) -> Option<Ref<'_, u64, Arc<Session>>> {
        self.session_map
            .get(&Session::global_id(gateway_id, session_id))
    }

    pub fn remove(&self, gateway_id: u32, session_id: u32) -> Option<Arc<Session>> {
        self.session_map
            .remove(&Session::global_id(gateway_id, session_id))
            .map(|kv| kv.1)
    }
}
