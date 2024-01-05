use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use crate::{Error, Result, web};

pub fn routes() -> Router {
    Router::new()
        .route("/api/login",post(api_login))
}

async fn api_login(cookies: Cookies,payload: Json<LoginPayLoad>) -> Result<Json<Value>> {
    if (payload.username != "demo" || payload.pwd != "helcome") {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result":{
            "success":true
        }}));

        Ok(body)

}

#[derive(Debug,Deserialize)]
struct LoginPayLoad {
    username: String,
    pwd: String
}