#![allow(unused)]

mod error;
mod model;
mod web;
pub use self::error::{Error, Result};

use std::net::SocketAddr;
use axum::extract::Path;
use axum::response::{Html, IntoResponse, Response};
use axum::{middleware, Router};
use axum::routing::{get, get_service};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use crate::model::ModelController;

#[tokio::main]
async fn main() -> Result<()> {

    let mc = ModelController::new().await?;

    // region: ---Routes
    let routes = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    async fn main_response_mapper(res: Response) -> Response {
        println!("->> {:<12} - main_response_mapper","RES_MAPPER");

        println!();

        res
    }

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
