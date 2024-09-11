use dashmap::{mapref::one::RefMut, DashMap};

use crate::session::Session;

#[derive(Default)]
pub struct SessionManager {
    session_map: DashMap<u32, Session>,
}

impl SessionManager {
    pub fn add(&self, id: u32, session: Session) {
        self.session_map.insert(id, session);
    }

    pub fn get_mut(&self, id: u32) -> Option<RefMut<'_, u32, Session>> {
        self.session_map.get_mut(&id)
    }
}
