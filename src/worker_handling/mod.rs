use actix_web::rt;
use reqwest::blocking;

fn send_info_to_work() {

}

pub async fn handle_connection_to_workers() {
    
    

    loop {

        /*
        let client = blocking::Client::new().post("http://127.0.0.1:8080/job/new");
    
        rt::spawn( 
        async {
            let x = client.send();

            println!("{:?}", x.unwrap())
        }
    
        );
        */
    }
}