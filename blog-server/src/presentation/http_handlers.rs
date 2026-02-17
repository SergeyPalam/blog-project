use actix_web::{web, HttpResponse, Result, http::StatusCode};

use crate::application::blog_service;
use crate::infrastructure::AppState;
use crate::application::auth_service::*;
use crate::application::blog_service::*;
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

pub async fn create_post(auth_user: AuthUser, new_post: web::Json<CreatePostReq>, app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let new_post = new_post.into_inner();
    let blog_service = app_state.blog_service.clone();
    let resp_data = blog_service.create_post(auth_user, new_post).await?;
    Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(resp_data))
}