use actix_app::user_data::user::User;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use std::collections::HashMap;
use uuid::Uuid;

// If I had more time I would use Tokio's rwlock.
// using tokios rwlock would lowwer the amount of blocking
// and is what the actix docks recommends!!!
use std::sync::RwLock;

#[derive(Debug)]
struct AppState {
    users: RwLock<HashMap<Uuid, User>>,
}

#[actix_web::get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::get("/getuser/")]
async fn get_user(user_id: web::Json<Uuid>, data: web::Data<AppState>) -> HttpResponse {
    let users = data.users.read().unwrap();
    if let Some(user) = users.get(&user_id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::Ok().json(())
    }
}

#[actix_web::get("/get-all-users/")]
async fn get_all_users(data: web::Data<AppState>) -> HttpResponse {
    let users = data.users.read().unwrap();
    HttpResponse::Ok().json(users.clone())
}

#[actix_web::get("/createuser/")]
async fn create_user(data: web::Data<AppState>, user: web::Json<User>) -> HttpResponse {
    let mut users = data.users.write().unwrap();
    users.insert(user.id, user.clone());

    // Return user added
    HttpResponse::Ok().json(user)
}

async fn serve_user_form() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/form.html")))
}

// This end point is just to make adding users easier
#[actix_web::post("/create-user-form/")]
async fn create_user_form(data: web::Data<AppState>, user: web::Form<User>) -> Result<String> {
    let user = user.into_inner();
    //TODO get rid of unwrap
    let mut users = data.users.write().unwrap();
    users.insert(user.id, user.clone());

    Ok(format!(
        "Welcome {},\nYour User profile is:\n{:?}",
        user.username, user
    ))
}

#[actix_web::get("/deleteuser/")]
async fn delete_user(user_id: web::Json<Uuid>, data: web::Data<AppState>) -> HttpResponse {
    let user_id = user_id.into_inner();
    let mut users = data.users.write().unwrap();
    if let Some(_) = users.remove(&user_id) {
        HttpResponse::Ok().body(format!("Deleting user {}!", user_id))
    } else {
        // Not sure if this is the correct responce code. This would be
        // something I would ask/see if it comes up in code review.
        HttpResponse::BadRequest().body(format!("No such user Exists: {}", user_id))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        users: RwLock::new(HashMap::new()),
    });
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("For Now checkout '/create-user-form/' and '/get-all-users/' ");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(state.clone())
            .service(health)
            .service(get_user)
            .service(get_all_users)
            .service(create_user)
            .service(create_user_form)
            .service(delete_user)
            .service(web::resource("/create-user-form/").route(web::get().to(serve_user_form)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
