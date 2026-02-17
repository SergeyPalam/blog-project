use actix_web::{Error, HttpMessage, dev::{Service, ServiceRequest, ServiceResponse, Transform}, web};
use actix_web::http::header;
use std::future::{ready, Ready, Future};
use std::{pin::Pin, task::{Context, Poll}};

use crate::infrastructure::AppState;
use crate::domain::error;

pub struct Jwt;

impl<S, B> Transform<S, ServiceRequest> for Jwt
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuth<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuth { service }))
    }
}
pub struct JwtAuth<S> { service: S }

impl<S, B> Service<ServiceRequest> for JwtAuth<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(str::to_owned);

        let Some(token) = token else {
            return Box::pin(async move { Err(error::AppError::Unauthorized("missing bearer".to_string()).into())});
        };

        let jwt_service = if let Some(data) = req.app_data::<web::Data<AppState>>(){
            data.jwt_service.clone()
        }else{
            return Box::pin(async move { Err(error::AppError::InternalError(String::new()).into())});
        };

        let Some(claims) = jwt_service.verify_token(&token) else{
            return Box::pin(async move { Err(error::AppError::Unauthorized("invalid credentials".to_string()).into())});
        };

        req.extensions_mut().insert(claims);

        let handler_service = self.service.call(req);

        Box::pin(async move {
            handler_service.await
        })
    }
} 