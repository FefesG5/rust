use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use env_logger;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}
#[derive(Deserialize)]
struct Numbers {
    numbers: Vec<f64>,
}

async fn index(info: web::Json<Info>) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body(format!("Welcome {}!", info.username)))
}

async fn print_numbers(numbers: web::Json<Numbers>) -> HttpResponse {
    println!("Received numbers: {:?}", numbers.numbers);
    HttpResponse::Ok().finish()
}

fn kahan_sum(numbers: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut c = 0.0;
    for &num in numbers{
        let y = num - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}

fn calculate_mean(numbers: &[f64]) -> f64 {
    let sum = kahan_sum(numbers);
    sum / (numbers.len() as f64)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("This server is running")
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(index))
            .service(hello)
            .service(web::resource("/numbers").route(web::post().to(print_numbers)))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}