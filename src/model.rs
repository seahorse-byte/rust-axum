// simplistic model
// with mock store layer

use crate::{
    ctx::Ctx,
    error::{Error, Result},
};
use serde::{de, Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// TICKET TYPES
#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64, // creator user_id
    pub title: String,
}
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// MODEL CONTROLLER

#[derive(Clone)]
pub struct ModelController {
    tickers_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickers_store: Arc::default(),
        })
    }
}

// CRUD

impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickers_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickers_store.lock().unwrap();
        let tickets = store
            .iter()
            .filter_map(|ticket| ticket.clone())
            .collect::<Vec<_>>();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickers_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|ticket| ticket.take());
        ticket.ok_or(Error::TicketDeleteFailIDNotFound { id })
    }

    // pub async fn get_ticket(&self, id: u64) -> Result<Ticket> {
    //     let store = self.tickers_store.lock().unwrap();
    //     let ticket = store
    //         .get(id as usize)
    //         .ok_or(Error::TicketNotFound)?
    //         .clone()
    //         .ok_or(Error::TicketNotFound)?;
    //     Ok(ticket)
    // }

    // pub async fn update_ticket(&self, id: u64, ticket: TicketForCreate) -> Result<Ticket> {
    //     let mut store = self.tickers_store.lock().unwrap();
    //     let ticket = store
    //         .get_mut(id as usize)
    //         .ok_or(Error::TicketNotFound)?
    //         .as_mut()
    //         .ok_or(Error::TicketNotFound)?;
    //     ticket.title = ticket.title.clone();
    //     Ok(ticket.clone())
    // }
}
