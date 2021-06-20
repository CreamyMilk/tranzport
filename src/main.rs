use actix_web::{
     middleware, web, App, HttpRequest, HttpResponse, HttpServer,
};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

/// This handler uses json extractor
async fn index(item: web::Json<MyObj>,req :HttpRequest) -> HttpResponse {
    println!("Automajically extacted model: {:?}", &item);
    println!("request: {:?}", req.headers());

    HttpResponse::Ok().json(item.0) // <- send response
}


/// This handler uses json extractor
async fn index2(item: web::Json<MyObj>,req :HttpRequest ) -> HttpResponse {
    let m = MyObj{name:"naming".to_string(),number:100};
    let pp = [m];
    println!("request: {:?}", req);
    println!("model: {:?}", item);
    HttpResponse::Ok().json(pp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/").route(web::post().to(index)))
            .service(web::resource("/a").route(web::post().to(index2)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
