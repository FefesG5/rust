use crate::*;
use actix_web::{test, App, http::StatusCode};

#[actix_web::test]
async fn test_server_availability(){
    let mut app = test::init_service(
        App::new().service(crate::check_server_status)
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_static_html(){
    let mut app = test::init_service(
        App::new()
            .service(Files::new("/", "./static").index_file("index.html"))
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;

    let body = test::read_body(resp).await;
    assert!(std::str::from_utf8(&body).unwrap().contains("Statistical Calculator"));
}

#[actix_web::test]
async fn test_return_calculations() {
    let mut app = test::init_service(
        App::new().service(
            web::resource("/numbers").route(web::post().to(return_calculations))
        )
    ).await;

    let test_payload = r#"{
        "numbers": [1.0, 2.0, 3.0, 4.0, 5.0],
        "is_population": false
    }"#;

    use actix_web::http::header::{self, HeaderValue};

    let req = test::TestRequest::post()
    .uri("/numbers")
    .insert_header((header::CONTENT_TYPE, HeaderValue::from_static("application/json")))
    .set_payload(test_payload)
    .to_request();


    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    let json_response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json_response["mean"], 3.0);
}
