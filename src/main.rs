use actix_web::{web, App, HttpRequest, HttpResponse ,HttpServer};
use actix_web::middleware::Logger;
use env_logger;
use askama::Template;

#[derive(Template)]
#[template(path="index.html")]
struct ServicesIndex<'a>{
    services_list: &'a std::vec::Vec<ServicesStatus>
}

struct ServicesStatus {
    status_code: String,
    service_name: String
}

impl std::fmt::Display for ServicesStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.status_code, self.service_name)
    }
}

fn services_status(_req: HttpRequest) -> HttpResponse{
    let services = [
        "https://appgofit.com.br/",
        "https://gestao.appgofit.com.br/",
        "https://institutobehappier.com.br/",
        "https://adm.movimentomusical.com.br/",
        "https://guiadaorganizacao.com.br/",
        "https://monicaponde.com.br/"
    ];
    
    let mut final_data = Vec::new();

    for service in &services {
        let status = reqwest::get(service.to_owned()).unwrap()
        .status();

        final_data.push(ServicesStatus{status_code: status.to_string(), service_name: service.to_string()});
    };
    
    let resp = ServicesIndex{
        services_list: &final_data
    }.render().unwrap();

    HttpResponse::Ok().body(resp)
}

fn main() {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/status", web::get().to(services_status))
    })
    .bind("0.0.0.0:8080")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
