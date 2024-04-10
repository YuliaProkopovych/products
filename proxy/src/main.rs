//! Run with
//!
//! ```not_rust
//! $ cargo run -p example-http-proxy
//! ```
//!
//! In another terminal:
//!
//! ```not_rust
//! $ curl -v -x "127.0.0.1:3000" https://tokio.rs
//! ```
//!
//! Example is based on <https://github.com/hyperium/hyper/blob/master/examples/http_proxy.rs>

use axum::{
    body::Body,
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

// use hyper::{server::Server, Request};
use reqwest;
use serde::Deserialize;
use std::convert::Infallible;

use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::upgrade::Upgraded;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tower::Service;
use tower::ServiceExt;

use hyper_util::rt::TokioIo;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Deserialize)]
struct UrlParam {
    pub url: String,
}

async fn fetch_page(url: String) -> String {
    //let url = &params.url;
    // Fetch the page content from the specified URL
    let response = reqwest::get(url)
        .await
        .expect("Failed to fetch page")
        .text()
        .await
        .expect("Failed to read page content");

    // Return the fetched page content
    response
}
use regex::Regex;
use reqwest::Url;
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

// async fn fetch_page_and_extract_links(url: &str) -> Result<(String, Vec<String>, Vec<String>), reqwest::Error> {
//     let response = reqwest::get(url).await?;
//     let body = response.text().await?;
    
//     let mut css_links = vec![];
//     let mut js_links = vec![];

//     let document = select::document::Document::from(body.as_str());
//     for node in document.find(select::predicate::Attr("href", "")) {
//         if let Some(link) = node.attr("href") {
//             if link.ends_with(".css") {
//                 css_links.push(link.to_string());
//             }
//         }
//     }
//     for node in document.find(select::predicate::Name("script")) {
//         if let Some(link) = node.attr("src") {
//             if link.ends_with(".js") {
//                 js_links.push(link.to_string());
//             }
//         }
//     }

//     Ok((body, css_links, js_links))
// }

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

    // println!("Server running on {}", addr);

    // if let Err(e) = server.await {
    //     eprintln!("server error: {}", e);
    // }

    // let router_svc = Router::new().route("/", get(|| async { "Hello, World!" }));

    // let tower_service = tower::service_fn(move |req: Request<_>| {
    //     let router_svc = router_svc.clone();
    //     let req = req.map(Body::new);
    //     async move {
    //         if req.method() == Method::CONNECT {
    //             proxy(req).await
    //         } else {
    //             router_svc.oneshot(req).await.map_err(|err| match err {})
    //         }
    //     }
    // });

    // let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
    //     tower_service.clone().call(request)
    // });

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("listening on {}", addr);

    // let listener = TcpListener::bind(addr).await.unwrap();
    // loop {
    //     let (stream, _) = listener.accept().await.unwrap();
    //     let io = TokioIo::new(stream);
    //     let hyper_service = hyper_service.clone();
    //     tokio::task::spawn(async move {
    //         if let Err(err) = http1::Builder::new()
    //             .preserve_header_case(true)
    //             .title_case_headers(true)
    //             .serve_connection(io, hyper_service)
    //             .with_upgrades()
    //             .await
    //         {
    //             println!("Failed to serve connection: {:?}", err);
    //         }
    //     });
    // }
}

// async fn proxy(req: Request) -> Result<Response, hyper::Error> {
//     tracing::trace!(?req);

//     if let Some(host_addr) = req.uri().authority().map(|auth| auth.to_string()) {
//         tokio::task::spawn(async move {
//             match hyper::upgrade::on(req).await {
//                 Ok(upgraded) => {
//                     if let Err(e) = tunnel(upgraded, host_addr).await {
//                         tracing::warn!("server io error: {}", e);
//                     };
//                 }
//                 Err(e) => tracing::warn!("upgrade error: {}", e),
//             }
//         });

//         Ok(Response::new(Body::empty()))
//     } else {
//         tracing::warn!("CONNECT host is not socket addr: {:?}", req.uri());
//         Ok((
//             StatusCode::BAD_REQUEST,
//             "CONNECT must be to a socket address",
//         )
//             .into_response())
//     }
// }

// async fn tunnel(upgraded: Upgraded, addr: String) -> std::io::Result<()> {
//     let mut server = TcpStream::connect(addr).await?;
//     let mut upgraded = TokioIo::new(upgraded);

//     let (from_client, from_server) =
//         tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

//     tracing::debug!(
//         "client wrote {} bytes and received {} bytes",
//         from_client,
//         from_server
//     );

//     Ok(())
// }