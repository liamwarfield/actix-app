use actix_app::show::show_config;
use actix_app::state::AppState;
use actix_app::ticket::ticket_config;
use actix_app::user::user_config;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::collections::HashMap;
use std::sync::RwLock;

// If I had more time I would use Tokio's rwlock.
// using tokios rwlock would lowwer the amount of blocking
// and is what the actix docks recommends!!!

#[actix_web::get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        users: RwLock::new(HashMap::new()),
        shows: RwLock::new(HashMap::new()),
        tickets: RwLock::new(HashMap::new()),
    });
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("For Now checkout '/create-user-form/' and '/get-all-users/' ");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(state.clone())
            .service(health)
            .service(web::scope("/user").configure(user_config))
            .service(web::scope("/show").configure(show_config))
            .service(web::scope("/ticket").configure(ticket_config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
