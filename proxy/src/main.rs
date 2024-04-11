use axum::{
    body::Body,
    http::{Method, StatusCode},
    response::Response,
    routing::get,
    Router,
};

use regex::Regex;
use reqwest;
use serde::Deserialize;
use std::convert::Infallible;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Deserialize)]
struct UrlParam {
    pub url: String,
}

async fn handle_request(url: axum::extract::Query<UrlParam>,req: axum::extract::Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::OPTIONS, _) => {
            Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .header(hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header(hyper::header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS")
                .header(hyper::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type")
                .header(hyper::header::ACCESS_CONTROL_MAX_AGE, "3600")
                .body(Body::empty())
                .unwrap())
        },
        (&Method::GET, _) => {
            let response = reqwest::get(url.url.clone()).await.unwrap();
            let body = response.text().await.unwrap();
        
            let proxy_url = "http://localhost:8000/?url=";
            let re = Regex::new(r#"((src|href)=['"])([^'"]+)['"]"#).unwrap();
            let rewritten_body = re.replace_all(&body, |caps: &regex::Captures| {
                format!("{}{}{}", &caps[1], proxy_url, &caps[3])
            });

            let mut response = Response::new(Body::from(rewritten_body.to_string()));
            (*response.headers_mut())
                .insert(hyper::header::CONTENT_TYPE, "text/html".parse().unwrap());
            Ok(response)
        },
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header(hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header(hyper::header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS")
                .header(hyper::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type")
                .body(Body::empty())
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_http_proxy=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create a new Axum router
    let app = Router::new().route("/", get(handle_request));

    // Define the address to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // Start the Hyper server
    axum::serve(listener, app).await.unwrap();

}
