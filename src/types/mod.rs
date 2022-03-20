use actix_web::web;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct AppState {
    pub job_queue: Arc<Mutex<Vec<(Uuid, web::Json<JobInfo>)>>>,

    pub workers_info: Arc<Mutex<Vec<web::Json<WorkerInfo>>>>,
}

#[derive(Deserialize)]
pub struct JobInfo {
    pub name: String,
    pub job_type: String,
}

#[derive(Deserialize)]
pub struct WorkerInfo {}
