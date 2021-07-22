use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::usize;
use uuid::Uuid;

use crate::payments::{has_payment_been_processed, start_payment_processing, PaymentService};
use crate::state::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Ticket {
    id: Uuid,
    user_id: Uuid,
    show_id: Uuid,
    // Number of cents. I am not a finace code expert.
    // I know that storing money as a float is a BAD idea
    // and thats it. In real life I would get help for this
    // or leave it up to postgres to handle money information.
    // I am not going to make an ACID transaction systeme for
    // this interview.
    // Just in case this gets posted somewhere: DO NOT use this
    // code to actaully move around money.
    price: usize,
    purchase_time: DateTime<Utc>,
    payment_service_token: PaymentService,
}

#[actix_web::get("/get")]
async fn get_ticket(ticket_id: web::Json<Uuid>, data: web::Data<AppState>) -> HttpResponse {
    let tickets = data.tickets.read().unwrap();
    if let Some(ticket) = tickets.get(&ticket_id) {
        HttpResponse::Ok().json(ticket)
    } else {
        HttpResponse::Ok().json(())
    }
}

// Frontend needs to call this before calling create_ticket
#[actix_web::get("/startpaymentprocessing/")]
async fn create_user(payment_token: web::Json<PaymentService>) -> HttpResponse {
    start_payment_processing(payment_token.into_inner());
    HttpResponse::Accepted().json("Call back in a few seconds")
}

#[actix_web::get("/create/")]
async fn create_ticket(data: web::Data<AppState>, ticket: web::Json<Ticket>) -> HttpResponse {
    let mut tickets = data.tickets.write().unwrap();
    let ticket = ticket.into_inner();
    match has_payment_been_processed(&ticket.payment_service_token) {
        Ok(_) => {
            tickets.insert(ticket.id, ticket.clone());
            HttpResponse::Accepted().json("Ticket Accepted")
        }
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

#[actix_web::get("/delete/")]
async fn delete_ticket(data: web::Data<AppState>, ticket_id: web::Json<Uuid>) -> HttpResponse {
    let mut tickets = data.tickets.write().unwrap();
    let ticket_id = ticket_id.into_inner();

    if let Some(ticket) = tickets.remove(&ticket_id) {
        return HttpResponse::Ok().json(ticket);
    } else {
        HttpResponse::BadRequest().body("No such ticket exists")
    }
}

pub fn ticket_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_ticket)
        .service(create_ticket)
        .service(delete_ticket);
}
