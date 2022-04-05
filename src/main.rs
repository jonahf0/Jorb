mod handlers;
mod types;
mod worker_handling;

use handlers::*;
use types::*;
use worker_handling::*;

use actix_web::middleware::Logger;
use actix_web::{rt, web, App, HttpServer};
//use std::sync::{Arc, Mutex};
use sled::open;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //starts logging to terminal
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //open the db
    let jorb_db = match open("jorb.db") {
        Ok(db) => { db },
        Err(e) => { panic!("failed to open jorb.db: {:?}", e) }
    };

    //open each of the trees
    let job_queue = match jorb_db.open_tree("job_queue") {
        Ok(tree) => { tree },
        Err(e) => { panic!("failed to open job_queue: {:?}", e) }
    };

    let results = match jorb_db.open_tree("results_tree") {
        Ok(tree) => { tree },
        Err(e) => { panic!("failed to open results_tree: {:?}", e) }
    };

    let worker_info = match jorb_db.open_tree("worker_info") {
        Ok(tree) => { tree },
        Err(e) => { panic!("failed to open worker_info: {:?}", e) }
    };

    //create an instance of AppState;
    let state_data = web::Data::new(AppState {
        job_queue,

        worker_info,

        results
    });

    rt::spawn(handle_connection_to_workers(state_data.clone()));

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
