pub mod application;
pub mod data;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use tonic::transport::Server;

use actix_cors::Cors;
use actix_web::{App, HttpServer, guard, web};
use anyhow::{Result, bail};
use tracing;
use tracing_actix_web::TracingLogger;

use presentation::grpc_service::{BlogGrpcService, proto::blog_service_server::BlogServiceServer};
use presentation::http_handlers::*;
use presentation::middleware;

#[actix_web::main]
async fn main() -> Result<()> {
    let app_state = match infrastructure::init().await {
        Ok(val) => web::Data::new(val),
        Err(e) => {
            eprint!("Can't init server: {e}");
            bail!("{e}");
        }
    };

    let grpc_addr = "0.0.0.0:50051".parse()?;
    let grpc_service = BlogGrpcService::new(app_state.clone());

    tokio::spawn(async move {
        tracing::info!("Blog gRPC server starting on {}", grpc_addr);
        Server::builder()
            .add_service(BlogServiceServer::new(grpc_service))
            .serve(grpc_addr)
            .await
            .unwrap();
        tracing::info!("Blog gRPC server finished");
    });

    tracing::info!("Start http server");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(TracingLogger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(register))
                            .route("/login", web::post().to(login)),
                    )
                    .service(
                        web::scope("/posts")
                            .guard(guard::Any(guard::Post()).or(guard::Put()).or(guard::Delete()))
                            .wrap(middleware::Jwt)
                            .route("", web::post().to(create_post))
                            .route("/{id}", web::put().to(update_post))
                            .route("/{id}", web::delete().to(delete_post)),
                        )
                    .service(
                        web::scope("/posts")
                            .guard(guard::Get())
                            .route("", web::get().to(get_posts))
                            .route("/{id}", web::get().to(get_post)),
                    )
            )
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await?;
    tracing::info!("Http server finished");
    Ok(())
}
