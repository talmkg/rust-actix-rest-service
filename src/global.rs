use std::sync::{Arc, Mutex};
use tokio_postgres::Client;

pub struct AppStateWithCounter {
    pub client: Arc<Mutex<Client>>,
}
