use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
mod reconst;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseStruct {
    pub message_dump: Vec<MessageDump>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDump {
    pub subscriptionid: String,
    pub message_timestamp: String,
    pub thread: String,
    pub phone_number: String,
    #[serde(rename = "MessageSubject")]
    pub message_subject: String,
    pub messagetxt: String,
}

impl MessageDump {
    fn classify(&self) {
        println!("Buda hii ni {}", self.message_timestamp);
    }
}

async fn message_handler(m: web::Json<ResponseStruct>, _req: HttpRequest) -> HttpResponse {
    for x in m.message_dump.iter() {
        x.classify();
    }
    let res = MyObj{message:"Guess Thanks".to_string()};
    HttpResponse::Ok().json(res)
}

async fn hello_handler(_req: HttpRequest) -> HttpResponse {
    let m = MyObj {
        message: "We Runninng my broder".to_string(),
    };
    HttpResponse::Ok().json(m)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", reconst::ISDEPOSIT);
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096000))
            .service(web::resource("/").route(web::get().to(hello_handler)))
            .service(web::resource("/data").route(web::post().to(message_handler)))
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await
}
