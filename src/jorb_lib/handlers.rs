
use super::types::*;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::{Utc, Timelike};
use bincode::serialize;

//the route for submitting jobs to the queue
pub async fn submit_job(state: web::Data<AppState>, job_info: web::Json<NewJobInfo>) -> HttpResponse {

    //get the time that this request is made
    let time_now = Utc::now();

    let time_string = format!(
        "{}-{}-{}-{}",
        time_now.hour(),
        time_now.minute(), 
        time_now.second(),
        time_now.nanosecond()
    );

    //create a JobInfo object from the NewJobInfo that's in the body;
    //if the user tries to send something that's not NewJobInfo, the user gets
    //a BadRequest by default
    let new_job_info = JobInfo {
        time: time_string.clone(),
        uuid: Uuid::new_v4(),
        name: job_info.name.clone(),
        job_type: job_info.job_type.clone(),
        arguments: job_info.arguments.clone(),
        file: job_info.arguments.clone()
    };

    //try to serialize
    let serialized_job_info = match serialize(&new_job_info) {
        
        Ok(byte_array) => { byte_array },
        
        Err(e) => { 
            println!("ERROR: failed to serialize new_job_info: {:?}", e);
            return HttpResponse::InternalServerError().body("could not deserialize the job information")
        }

    };

    //try to insert the NewJobInfo corresponding to 
    if let Err(e) = state.job_queue.insert(time_string.clone(), serialized_job_info) {
        println!("ERROR: failed to insert new_job_info: {:?}", e);
        return HttpResponse::InternalServerError().body("could not insert the new")
    }

    HttpResponse::Ok().body(new_job_info.uuid.to_string())
}

//the route for submitting jobs to the queue
pub async fn cancel_job(
    state: web::Data<AppState>,
    job_uuid_string: web::Path<String>,
) -> HttpResponse {

    //get the uuid that was used
    let job_uuid = Uuid::parse_str(&job_uuid_string).unwrap();
    /* 
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
    */
    HttpResponse::Ok().body("success")
}
