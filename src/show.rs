use crate::state::AppState;
use crate::user::Role;
use actix_web::{web, HttpResponse};
use chrono::{prelude::*, DateTime};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Show {
    pub id: Uuid,
    pub title: String,
    pub url: Url,
    #[serde(default = "Utc::now")]
    pub showtime: DateTime<Utc>,
}

// Only hash the UUID
impl Hash for Show {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Show {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Show {}

#[actix_web::get("/get/")]
async fn get_show(show_id: web::Json<Uuid>, data: web::Data<AppState>) -> HttpResponse {
    let shows = data.shows.read().unwrap();
    if let Some(show) = shows.get(&show_id) {
        HttpResponse::Ok().json(show)
    } else {
        HttpResponse::Ok().json(())
    }
}

// Here I use the user's id for validation, but in reality
// This would take in some type of session/JWT token and do
// real auth.
#[actix_web::get("/create/")]
async fn create_show(data: web::Data<AppState>, info: web::Json<(Uuid, Show)>) -> HttpResponse {
    let mut shows = data.shows.write().unwrap();
    let users = data.users.read().unwrap();
    let (user_id, show) = info.into_inner();

    if let Some(user) = users.get(&user_id) {
        if matches!(user.role, Role::Admin) {
            shows.insert(show.id, show.clone());
            return HttpResponse::Ok().json(show);
        } else {
            HttpResponse::Unauthorized().json("User Lacks authority to create a show")
        }
    } else {
        HttpResponse::Unauthorized().json("User Lacks authority to create a show")
    }
}

#[actix_web::get("/delete/")]
async fn delete_show(
    data: web::Data<AppState>,
    user_id: web::Json<Uuid>,
    show_id: web::Json<Uuid>,
) -> HttpResponse {
    let mut shows = data.shows.write().unwrap();
    let users = data.users.read().unwrap();
    let show_id = show_id.into_inner();

    if let Some(user) = users.get(&user_id.into_inner()) {
        if matches!(user.role, Role::Admin) {
            if let Some(show) = shows.remove(&show_id) {
                return HttpResponse::Ok().json(show);
            } else {
                HttpResponse::BadRequest().json("No such show exists")
            }
        } else {
            HttpResponse::Unauthorized().json("User Lacks authority to create a show")
        }
    } else {
        HttpResponse::Unauthorized().json("User Lacks authority to create a show")
    }
}

pub fn show_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_show)
        .service(create_show)
        .service(delete_show);
}
