
use super::types::*;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::{Utc, Timelike, Datelike};
use bincode::{serialize, deserialize};

//the route for submitting jobs to the queue
pub async fn submit_job(state: web::Data<AppState>, job_info: web::Json<NewJobInfo>) -> HttpResponse {

    //get the time that this request is made
    /*let time_now = Utc::now();

    let date_component = time_now.date();

    let time_string = format!(
        "{}{}{}{}{}{}",
        date_component.year(),
        date_component.month(),
        date_component.day(),
        time_now.hour(),
        time_now.minute(),
        time_now.second(),
        time_now.tim
    );*/

    //create a JobInfo object from the NewJobInfo that's in the body;
    //if the user tries to send something that's not NewJobInfo, the user gets
    //a BadRequest by default
    let new_job_info = JobInfo {
        time: Utc::now().timestamp_millis().to_string(),
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
    if let Err(e) = state.job_queue.insert(new_job_info.time, serialized_job_info) {
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

    /*
    /*
    //try to insert the NewJobInfo corresponding to 
    if let Err(e) = state.job_queue.insert(time_string.clone(), serialized_job_info) {
        println!("ERROR: failed to insert new_job_info: {:?}", e);
        return HttpResponse::InternalServerError().body("could not insert the new")
    }*/

    let serialized_job_info = match state.job_queue.get()

    //try to serialize
    let job_info = match deserialize(&new_job_info) {

        Ok(byte_array) => { byte_array },

        Err(e) => { 
            println!("ERROR: failed to serialize new_job_info: {:?}", e);
            return HttpResponse::InternalServerError().body("could not deserialize the job information")
        }

    };
    */
    HttpResponse::Ok().body("success")
}

pub async fn inspect_tree( state: web::Data<AppState>) -> HttpResponse {

    let tree_iter = state.job_queue.into_iter();

    for item in tree_iter {

        if let Ok(byte_val) = item {
            
            let deserialized: Result<JobInfo, bincode::Error> = deserialize::<JobInfo>(&byte_val.1);        

            match deserialized {
                Ok(val) => { println!("{:?} -- {:?} {:?}", val.time, val.uuid, val.name)},
                Err(e) => {println!("{:?}", e)}
            }

        }
    }

    HttpResponse::Ok().body("success")

}
