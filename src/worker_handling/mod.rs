use crate::types::{AppState, JobInfo, WorkerInfo};

use actix_web::{rt, web};
use reqwest::blocking;
use uuid::Uuid;

pub async fn handle_connection_to_workers(state_data: web::Data<AppState>) {
    loop {
        let mut queue = state_data.job_queue.try_lock();

        if let Ok(ref mut mutex) = queue {

            if let Some(info) = mutex.pop() {
                send_info(info).await
            }
        }
    }
}

async fn send_info(info: (Uuid, web::Json<JobInfo>)) {
    println!("{:?}", info.1.arguments);
    println!("{:?}", info.1.name);
}

async fn find_worker_info(workers: Vec<WorkerInfo>) {}
