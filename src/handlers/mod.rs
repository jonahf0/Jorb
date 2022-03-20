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

            mutex.push((job_info_uuid, job_info));

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

    //get the uuid that was used
    let job_uuid = Uuid::parse_str(&job_uuid_string).unwrap();

    loop {
        let mut queue = state.job_queue.try_lock();

        if let Ok(ref mut mutex) = queue {
        
            //search for the item; if it's there, then get rid of it
            let index = mutex.iter().position(|item| item.0 == job_uuid);

            if let Some(val) = index {
                mutex.remove(val);
            }

            break;
        }
    }

    HttpResponse::Ok().body("success")
}
