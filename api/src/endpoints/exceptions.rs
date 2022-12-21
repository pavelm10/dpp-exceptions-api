use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::serde::Serialize;
use rocket::{http::ContentType, http::Status, response, serde::json};
use std::io::Cursor;
use thiserror::Error;

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

#[allow(dead_code)]
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    DBError(String),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'o> {
        let err_response = json::to_string(&ErrorResponse {
            message: self.to_string(),
        })
        .unwrap();

        let status = match self {
            Error::Internal(_) => Status::InternalServerError,
            Error::NotFound(_) => Status::NotFound,
            Error::DBError(_) => Status::FailedDependency,
            _ => Status::BadRequest,
        };
        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(err_response.len(), Cursor::new(err_response))
            .ok()
    }
}
