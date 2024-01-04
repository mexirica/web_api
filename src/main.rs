#![allow(unused)]

mod error;
mod web;
pub use self::error::{Error, Result};

use std::net::SocketAddr;
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::{get, get_service};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // region: ---Routes
    let routes = Router::new().merge(routes_hello()).fallback_service(routes_static());

    fn routes_static() -> Router {
        Router::new().nest_service("/", get_service(ServeDir::new("./")))
    }
    fn routes_hello() -> Router {
        Router::new()
            .route(
            "/hello/:name",
            get(hello))
    }

    struct HelloParams {
        name: Option<String>
    }

    async fn hello(Path(name): Path<String>) -> impl IntoResponse {
        format!("Hello {}",name)
    }
    // endregion: ---Routes

    // region: ---Start Server
    let addr = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(addr,routes.into_make_service()).await.unwrap()
    // endregion: ---Start Server
}
