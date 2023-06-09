//! simple model layer for fun

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};


// region - ticket types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String
}

#[derive(Deserialize)]
pub struct TicketForCreate {
   pub title: String,
}
// endregion


// ModelController
#[derive(Clone, Debug)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Ticket>>>,
}

// constructor

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD implementation
impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)

        todo!()
    }
}