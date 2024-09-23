// src/main.rs

mod db;
mod models;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponderError};
use serde_json::json;

use crate::db::Database;

async fn create_user_profile(
    user: web::Json<models::UserProfile>,
    db: web::Data<web::Json<Database>>,
) -> impl Responder {
    let mut database = db.into_inner().clone();
    database.insert_user_profile(user.into_inner());

    let response = json!({
        "status": "success",
        "message": "User profile created",
    });

    HttpResponse::Ok().json(response)
}

async fn swipe(_: HttpRequest) -> impl Responder {
    // Your implementation here (e.g., fetch potential matches and perform swiping logic).
    let response = json!({
        "status": "success",
        "message": "Swiped!",
    });

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = web::Json(Database::new());

    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .route("/create_user_profile", web::post().to(create_user_profile))
            .route("/swipe", web::post().to(swipe))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
