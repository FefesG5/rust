use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};

#[allow(unused_imports)]
use serde::Deserialize;

mod calculations;

use calculations::{calculate_mean, calculate_standard_deviation, calculate_median, calculate_percentile, calculate_interquartile_range,
calculate_range,
calculate_variance
};

#[derive(Deserialize)]
struct Numbers {
    numbers: Vec<f64>,
}

#[get("/")]
async fn check_server_status() -> impl Responder {
    HttpResponse::Ok().body("This server is running..")
}

async fn return_calculations(numbers: web::Json<Numbers>) -> HttpResponse {
    let mean = calculate_mean(&numbers.numbers);
    let standard_deviation = calculate_standard_deviation(&numbers.numbers, mean);
    let median = calculate_median(&numbers.numbers);
    let q1_percentile = calculate_percentile(&numbers.numbers, 25.0);
    let q3_percentile = calculate_percentile(&numbers.numbers, 75.0); 
    let interquartile_range = calculate_interquartile_range(&numbers.numbers);
    let range = calculate_range(&numbers.numbers);
    let variance = calculate_variance(&numbers.numbers, mean);

    println!("Received numbers: {:?}", numbers.numbers);
    println!("Basic Calculations");
    println!("Calculated mean: {:.}", mean);
    println!("Calculated standard deviation: {:.}", standard_deviation);

    let response_data = serde_json::json!({
        "receivedNumbers": numbers.numbers,
        "mean": mean,
        "standardDeviation": standard_deviation,
        "median": median,
        "q1Percentile": q1_percentile,
        "q3Percentile": q3_percentile,
        "interquartileRange": interquartile_range,
        "range": range,
        "variance": variance
    });

    HttpResponse::Ok().json(response_data)
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