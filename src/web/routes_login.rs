use axum::Json;
use serde::Deserialize;
use serde_json::Value;
use crate::{Error, Result};

async fn api_login(payload: Json<LoginPayLoad>) -> Result<Json<Value>> {

}

#[derive(Debug,Deserialize)]
struct LoginPayLoad {
    username: String,
    pwd: String
}