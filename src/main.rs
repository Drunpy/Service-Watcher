use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use askama::Template;
use env_logger;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::vec::Vec;

#[derive(Template)]
#[template(path = "index.html")]
struct ServicesIndex<'a> {
    services_list: &'a std::vec::Vec<ServicesStatus>,
}

struct ServicesStatus {
    status_code: String,
    service_name: String,
}

impl std::fmt::Display for ServicesStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.status_code, self.service_name)
    }
}

// TODO: READ THE FILE AND RETURN ROUTES
fn read_service_file() -> String {
    let mut _file = File::open("./services.txt")
        .ok()
        .expect("Could not find file");

    let mut str_buffer = String::new();

    _file
        .read_to_string(&mut str_buffer)
        .ok()
        .expect("File could not be read.");

    //println!("{:?}", _file);

    return str_buffer;
}

fn services_status(_req: HttpRequest) -> HttpResponse {
    let mut final_data = Vec::new();

    for service in read_service_file().split("\n") {
        let status = reqwest::get(&service.to_owned()).unwrap().status();

        final_data.push(ServicesStatus {
            status_code: status.to_string(),
            service_name: service.to_string(),
        });
    }

    let resp = ServicesIndex {
        services_list: &final_data,
    }
    .render()
    .unwrap();

    HttpResponse::Ok().body(resp)
}

fn main() {
    let server: String = String::from("0.0.0.0:");
    let port: &str = "8080";
    let full_server = server + port;

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/status", web::get().to(services_status))
    })
    .bind(full_server)
    .expect("Can not bind to this port")
    .run()
    .unwrap();
}
