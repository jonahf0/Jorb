use crate::types::*;
use actix_web::{web, HttpResponse};
use uuid::Uuid;

//the route for submitting jobs to the queue
pub async fn submit_job(state: web::Data<AppState>, job_info: web::Json<JobInfo>) -> HttpResponse {
    let job_info_uuid;

    loop {
        let mut queue = state.job_queue.try_lock();

        if let Ok(ref mut mutex) = queue {
            job_info_uuid = Uuid::new_v4();

            mutex.insert(job_info_uuid, job_info);

            break;
        }
    }

    HttpResponse::Ok().body(job_info_uuid.to_string())
}

//the route for submitting jobs to the queue
pub async fn cancel_job(
    state: web::Data<AppState>,
    job_uuid_string: web::Path<String>,
) -> HttpResponse {
    let job_uuid = Uuid::parse_str(&job_uuid_string).unwrap();

    loop {
        let mut queue = state.job_queue.try_lock();

        if let Ok(ref mut mutex) = queue {
            //mutex.push(job_info);
            if mutex.contains_key(&job_uuid) {
                mutex.remove_entry(&job_uuid);
            }

            break;
        }
    }

    HttpResponse::Ok().body("success")
}
