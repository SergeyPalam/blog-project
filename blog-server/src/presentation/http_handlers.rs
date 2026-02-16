use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::infrastructure::AppState;

#[derive(Deserialize)]
struct RegisterUserReq {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct RegisteredUser {
    id: i64,
    name: String,
    email: String,
    token: String,
}

async fn register(new_user: web::Json<RegisterUserReq>, app_state: web::Data<AppState>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Hello, World!"))
}