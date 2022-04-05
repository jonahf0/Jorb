use serde::{Deserialize, Serialize};
//use std::sync::{Arc, Mutex};
use uuid::Uuid;
use sled::Tree;

/*
pub struct AppState {
    pub job_queue: Arc<Mutex<Vec<(Uuid, web::Json<JobInfo>)>>>,

    pub workers_info: Arc<Mutex<Vec<web::Json<WorkerInfo>>>>,
}
*/
pub struct AppState {
    pub job_queue: Tree,
    pub worker_info: Tree
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
