use actix_web::web;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use leveldb::database::Database;

/*
pub struct AppState {
    pub job_queue: Arc<Mutex<Vec<(Uuid, web::Json<JobInfo>)>>>,

    pub workers_info: Arc<Mutex<Vec<web::Json<WorkerInfo>>>>,
}
*/
pub struct AppState {
    pub job_queue: Arc<Mutex<Database<i32>>>,
    pub worker_info:  Arc<Mutex<Database<i32>>>
}

#[derive(Serialize, Deserialize)]
pub struct JobInfo {
    pub name: String,
    pub job_type: String,
    pub arguments: Option<String>,
    pub file: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct WorkerInfo {
    pub name: String,
    pub job_type: String,
    pub host: String
}
