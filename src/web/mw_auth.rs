use std::ops::Neg;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;
use crate::web::AUTH_TOKEN;
use crate::{Error,Result};

pub async fn mw_require_auth<B> (cookies: Cookies,req: Request<B>, next: Next) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::TicketDeleteFailIdNotFound)?;

    Ok(next.run(req).await)
}