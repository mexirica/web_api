use std::fmt::Formatter;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    LoginFail,
    TicketDeleteFailIdNotFound {id: u64},
    AuthFailNoAuthTokenCookie,
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(),core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl IntoResponse for Error{
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}","INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}