use axum::{
    http::{header::HeaderName, HeaderMap, HeaderValue},
    routing::{get, options},
    Json, Router,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CoocooUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct RegRequest {
    msg: String,
    username: String,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/reg", options(with_headers).post(reg_user))
        .route("/login", options(with_headers).post(login));

    // run it with hyper on localhost:3000
    println!("Coocoo is running!");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn with_headers() -> HeaderMap {
    headers()
}

async fn reg_user(Json(params): Json<CoocooUser>) -> (HeaderMap, Json<RegRequest>) {
    let json = Json(RegRequest {
        msg: "Success!".to_string(),
        username: params.username,
    });

    (headers(), json)
}

async fn login(Json(params): Json<CoocooUser>) -> (HeaderMap, Json<CoocooUser>) {
    (headers(), Json(params))
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("access-control-allow-origin"),
        HeaderValue::from_static("*"),
    );
    headers.insert(
        HeaderName::from_static("access-control-allow-headers"),
        HeaderValue::from_static("*"),
    );
    headers.insert(
        HeaderName::from_static("access-control-allow-methods"),
        HeaderValue::from_static("*"),
    );

    headers
}
