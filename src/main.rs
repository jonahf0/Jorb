mod handlers;
mod types;
mod worker_handling;

use handlers::*;
use types::*;
use worker_handling::*;

use actix_web::middleware::Logger;
use actix_web::{rt, web, App, HttpServer};
use std::collections::HashMap;
use std::sync::{Mutex,Arc};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //starts logging to terminal
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let workers_info_arc = Arc::new(Mutex::new(HashMap::new()));

    //create an instance of AppState;
    let state_data = web::Data::new(AppState {
        job_queue: Mutex::new(HashMap::new()),

        workers_info: workers_info_arc,
    });

    rt::spawn( handle_connection_to_workers() );

    //run the server with the appropriate routes
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state_data.clone())
            .route("/job/new", web::post().to(submit_job))
            .route("/job/cancel/{uuid}", web::post().to(cancel_job))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
