use actix_web::{Error, HttpMessage, HttpResponse, HttpResponseBuilder, dev::{Service, ServiceRequest, ServiceResponse, Transform}, http::StatusCode, web};
use actix_web::http::header;
use std::future::{ready, Ready, Future};
use std::{pin::Pin, task::{Context, Poll}};

use crate::infrastructure::AppState;

pub struct Jwt;

impl<S> Transform<S, ServiceRequest> for Jwt
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuth<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuth { service }))
    }
}
pub struct JwtAuth<S> { service: S }

impl<S> Service<ServiceRequest> for JwtAuth<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let return_err = |req: ServiceRequest, mut builder: HttpResponseBuilder, err_msg| {
            let response = builder
                .json(serde_json::json!({"error": err_msg}));

            let service_resp = ServiceResponse::new(req.into_parts().0, response);
            return Box::pin(async move { Ok(service_resp)});
        };

        let token = req.headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(str::to_owned);

        let Some(token) = token else {
            return return_err(req, HttpResponse::Unauthorized(), "missing bearer");
        };

        let jwt_service = if let Some(data) = req.app_data::<web::Data<AppState>>(){
            data.jwt_service.clone()
        }else{
            return return_err(req, HttpResponse::InternalServerError(), "Internal server error");
        };

        let Some(claims) = jwt_service.verify_token(&token) else{
            return return_err(req, HttpResponse::Unauthorized(), "invalid credentials");
        };

        req.extensions_mut().insert(serde_json::to_string(&claims));

        let handler_service = self.service.call(req);

        Box::pin(async move {
            handler_service.await
        })
    }
} 