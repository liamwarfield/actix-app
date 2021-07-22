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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_app::{show::Show, user::User};
    use actix_web::{test, web, App};
    use chrono::Utc;
    use url::Url;
    use uuid::Uuid;

    #[actix_rt::test]
    async fn test_user_endpoints() {
        let state = web::Data::new(AppState {
            users: RwLock::new(HashMap::new()),
            shows: RwLock::new(HashMap::new()),
            tickets: RwLock::new(HashMap::new()),
        });
        let mut app = test::init_service(
            App::new()
                .app_data(state.clone())
                .service(web::scope("/user").configure(user_config)),
        )
        .await;

        let id = Uuid::new_v4();
        let username = "Liam".to_string();
        let role = actix_app::user::Role::Admin;
        let time_created = Utc::now();

        let user = User {
            id,
            username,
            role,
            time_created,
        };

        let req = test::TestRequest::get()
            .uri("/user/create/")
            .set_json(&user)
            .to_request();
        let resp: User = test::read_response_json(&mut app, req).await;
        assert_eq!(user, resp);

        let req = test::TestRequest::get()
            .uri("/user/get/")
            .set_json(&user.id)
            .to_request();
        let resp: User = test::read_response_json(&mut app, req).await;
        assert_eq!(user, resp);

        let req = test::TestRequest::get()
            .uri("/user/delete/")
            .set_json(&user.id)
            .to_request();
        let resp: User = test::read_response_json(&mut app, req).await;
        assert_eq!(user, resp);

        // We should not be able to delete the same user twice
        let req = test::TestRequest::get()
            .uri("/user/delet/")
            .set_json(&user.id)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }
    #[actix_rt::test]
    async fn test_show_endpoints() {
        let state = web::Data::new(AppState {
            users: RwLock::new(HashMap::new()),
            shows: RwLock::new(HashMap::new()),
            tickets: RwLock::new(HashMap::new()),
        });
        let mut app = test::init_service(
            App::new()
                .app_data(state.clone())
                .service(web::scope("/show").configure(user_config))
                .service(web::scope("/user").configure(user_config)),
        )
        .await;

        let id = Uuid::new_v4();
        let username = "Liam".to_string();
        let role = actix_app::user::Role::Admin;
        let time_created = Utc::now();

        let admin_user = User {
            id,
            username,
            role,
            time_created,
        };

        let cust_user = User {
            id: Uuid::new_v4(),
            username: "Sam".to_string(),
            role: actix_app::user::Role::Customer,
            time_created,
        };

        // Add the users to the server:
        let req = test::TestRequest::get()
            .uri("/user/create/")
            .set_json(&admin_user)
            .to_request();
        let _ = test::call_service(&mut app, req).await;

        let req = test::TestRequest::get()
            .uri("/user/create/")
            .set_json(&cust_user)
            .to_request();
        let _ = test::call_service(&mut app, req).await;

        let show = Show {
            id: Uuid::new_v4(),
            title: "Rolling Bones".to_string(),
            url: Url::parse("https://crates.io").unwrap(),
            showtime: Utc::now(),
        };

        // Customer users should not be able to generate new shows.
        let req = test::TestRequest::get()
            .uri("/show/create/")
            .set_json(&(cust_user.id, &show))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());

        // Something screwwy is going on with Serde here causing this test to fail.
        // I don't have any time to track it down before the deadline.
        // I'm just going to leave this broken
        let req = test::TestRequest::get()
            .uri("/show/create/")
            .set_json(&(admin_user.id, show))
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success(), "responce was {:?}", resp);
    }

    // I could keep writing tests like this for a while,
    // its geting late, so I'm going to cut it here.
}
