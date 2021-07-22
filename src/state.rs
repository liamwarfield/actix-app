use crate::show::Show;
use crate::ticket::Ticket;
use crate::user::User;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
pub struct AppState {
    pub users: RwLock<HashMap<Uuid, User>>,
    pub shows: RwLock<HashMap<Uuid, Show>>,
    pub tickets: RwLock<HashMap<Uuid, Ticket>>,
}
