
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sled::Tree;

pub struct AppState {
    pub job_queue: Tree,
    pub worker_info: Tree,
    pub results: Tree
}

#[derive(Serialize, Deserialize)]
pub struct JobInfo {
    pub time: String,
    pub uuid: Uuid,
    pub name: String,
    pub job_type: String,
    pub arguments: Option<String>,
    pub file: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct WorkerInfo {
    pub uuid: Uuid,
    pub name: String,
    pub job_type: String,
    pub host: String
}

#[derive(Serialize, Deserialize)]
pub struct NewJobInfo {
    pub name: String,
    pub job_type: String,
    pub arguments: Option<String>,
    pub file: Option<String>
}
