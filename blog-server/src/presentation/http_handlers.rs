use actix_web::{web, App, HttpServer, HttpResponse, Result, http::StatusCode};

use crate::infrastructure::AppState;
use crate::application::auth_service::*;
use crate::domain::error::AppError;

pub async fn register(new_user: web::Json<RegisterUserReq>, app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let req = new_user.into_inner();
    let auth_service = app_state.auth_service.clone();
    let resp_data = auth_service.register(req).await?;
    Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(resp_data))
}

pub async fn login(user: web::Json<LoginUserReq>, app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let req = user.into_inner();
    let auth_service = app_state.auth_service.clone();
    let resp_data = auth_service.login(req).await?;
    Ok(HttpResponse::Ok().status(StatusCode::OK).json(resp_data))
}