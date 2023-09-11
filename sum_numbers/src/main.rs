use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};


#[allow(unused_imports)]
use serde::Deserialize;

mod calculations;

use calculations::{
    round_to_decimal_places,
    calculate_mean, 
    calculate_sample_standard_deviation,
    calculate_median, 
    calculate_percentile, 
    calculate_interquartile_range,
    calculate_range, 
    calculate_variance, 
    calculate_coefficient_of_variation,
    calculate_skewness,
    calculate_mode
};

#[derive(Deserialize)]
struct Numbers {
    numbers: Vec<f64>,
}

#[get("/")]
async fn check_server_status() -> impl Responder {
    HttpResponse::Ok().body("This server is running..")
}

async fn return_calculations(mut numbers: web::Json<Numbers>) -> HttpResponse {
    let mean = calculate_mean(&numbers.numbers);
    let standard_deviation = calculate_sample_standard_deviation(&numbers.numbers, mean);
    let median = calculate_median(&mut numbers.numbers[..]);
    let q1_percentile = calculate_percentile(&numbers.numbers, 25.0);
    let q3_percentile = calculate_percentile(&numbers.numbers, 75.0); 
    let interquartile_range = calculate_interquartile_range(&numbers.numbers);
    let range = calculate_range(&numbers.numbers);
    let variance = calculate_variance(&numbers.numbers, mean);
    let coefficient_of_variation = calculate_coefficient_of_variation(mean, standard_deviation);

    let skewness = match calculate_skewness(&numbers.numbers, mean, standard_deviation) {
        Some(value) => round_to_decimal_places(value, 9),
        None => f64::NAN,
    };

    let mode = calculate_mode(&numbers.numbers);

    println!("Received numbers: {:?}", numbers.numbers);
    println!("Basic Calculations");
    println!("Calculated mean: {:.}", round_to_decimal_places(mean, 9));
    println!("Calculated standard deviation: {:.}", standard_deviation);

    let response_data = serde_json::json!({
        "receivedNumbers": numbers.numbers,
        "mean": round_to_decimal_places(mean, 9),
        "standardDeviation": round_to_decimal_places(standard_deviation, 9),
        "median": round_to_decimal_places(median, 9),
        "q1Percentile": round_to_decimal_places(q1_percentile, 9),
        "q3Percentile": round_to_decimal_places(q3_percentile, 9),
        "interquartileRange": round_to_decimal_places(interquartile_range, 9),
        "range": round_to_decimal_places(range, 9),
        "variance": round_to_decimal_places(variance, 9),
        "coefficient_of_variation": round_to_decimal_places(coefficient_of_variation, 9),
        "skewness": skewness,
        "mode": mode
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