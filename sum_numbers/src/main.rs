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

fn calculate_standard_deviation(numbers: &[f64], mean:f64) -> f64 {
    let mut sum_squared_diff = 0.0;
    for &num in numbers {
        let diff = num - mean;
        sum_squared_diff += diff * diff
    }
    let mean_squared_diff = sum_squared_diff / (numbers.len() as f64);
    mean_squared_diff.sqrt()
}

#[get("/")]
async fn check_server_status() -> impl Responder {
    HttpResponse::Ok().body("This server is running..")
}

async fn return_calculations(numbers: web::Json<Numbers>) -> HttpResponse {
    let mean = calculate_mean(&numbers.numbers);
    let standard_deviation = calculate_standard_deviation(&numbers.numbers, mean);
    println!("Received numbers: {:?}", numbers.numbers);
    println!("Calculated mean: {:.}", mean);
    println!("Calculated standard deviation: {:.}", standard_deviation);

    let response_body = format!(
        "Received numbers: {:?}\nMean: {}\nStandard Deviation {}",
        numbers.numbers, mean, standard_deviation
    );

    HttpResponse::Ok().body(response_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(check_server_status)
            .service(web::resource("/numbers").route(web::post().to(return_calculations)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}