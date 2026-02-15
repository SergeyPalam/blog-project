use actix_web::{web, HttpServer, App};
use tracing;
use tracing_actix_web::TracingLogger;
use anyhow::{Result, bail};

pub mod domain;
pub mod infrastructure;

#[actix_web::main]
async fn main() -> Result<()>{
    let app_state = match infrastructure::init().await {
        Ok(val) => web::Data::new(val),
        Err(e) => {
            eprint!("Can't init server: {e}");
            bail!("{e}");
        }
    };
    
    tracing::info!("Start http server");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(TracingLogger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    tracing::info!("Http server finished");
    Ok(())
}
