use actix_web::web;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use uuid::Uuid;

pub struct AppState {
    pub job_queue: Mutex<HashMap<Uuid, web::Json<JobInfo>>>,

    pub workers_info: Arc<Mutex<HashMap<Uuid, web::Json<WorkerInfo>>>>,
}

#[derive(Deserialize)]
pub struct JobInfo {
    pub name: String,
    pub job_type: String,
}

#[derive(Deserialize)]
pub struct WorkerInfo {}
