use actix_web::{web, HttpResponse};
use chrono::{prelude::*, DateTime};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use uuid::Uuid;

use crate::state::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub role: Role,
    #[serde(default = "Utc::now")]
    pub time_created: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Admin,
    Customer,
}

// Only hash the UUID
impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

#[actix_web::get("/get/")]
async fn get_user(user_id: web::Json<Uuid>, data: web::Data<AppState>) -> HttpResponse {
    let users = data.users.read().unwrap();
    if let Some(user) = users.get(&user_id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::Ok().json(())
    }
}

#[actix_web::get("/get-all/")]
async fn get_all_users(data: web::Data<AppState>) -> HttpResponse {
    let users = data.users.read().unwrap();
    HttpResponse::Ok().json(users.clone())
}

#[actix_web::get("/create/")]
async fn create_user(data: web::Data<AppState>, user: web::Json<User>) -> HttpResponse {
    let mut users = data.users.write().unwrap();
    users.insert(user.id, user.clone());

    // Return user added
    HttpResponse::Ok().json(user)
}

async fn serve_user_form() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

// This end point is just to make adding users easier
#[actix_web::post("/create-user-form/")]
async fn create_user_form(
    data: web::Data<AppState>,
    user: web::Form<User>,
) -> actix_web::Result<String> {
    let user = user.into_inner();
    //TODO get rid of unwrap
    let mut users = data.users.write().unwrap();
    users.insert(user.id, user.clone());

    Ok(format!(
        "Welcome {},\nYour User profile is:\n{:?}",
        user.username, user
    ))
}

#[actix_web::get("/delete/")]
async fn delete_user(user_id: web::Json<Uuid>, data: web::Data<AppState>) -> HttpResponse {
    let user_id = user_id.into_inner();
    let mut users = data.users.write().unwrap();
    if let Some(user) = users.remove(&user_id) {
        HttpResponse::Ok().json(user)
    } else {
        // Not sure if this is the correct responce code. This would be
        // something I would ask/see if it comes up in code review.
        HttpResponse::BadRequest().body(format!("No such user Exists: {}", user_id))
    }
}

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user)
        .service(get_all_users)
        .service(create_user)
        .service(create_user_form)
        .service(delete_user)
        .service(web::resource("/create-user-form/").route(web::get().to(serve_user_form)));
}
