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