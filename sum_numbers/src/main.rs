use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger;

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

#[post("/echo")]
async fn echo(numbers: web::Json<Vec<f64>>) -> impl Responder {
    let num_list = &numbers;

    let average: f64 = calculate_mean(num_list);

    println!("Received numbers: {:?}", num_list);
    println!("Mean: {:.}", average);

    HttpResponse::Ok().json(average)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}