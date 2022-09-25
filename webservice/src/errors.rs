use actix_web::body::BoxBody;
use actix_web::{error, http::StatusCode, Error, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Datbase error".to_string()
            }
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".to_string()
            }
            MyError::NotFound(msg) => {
                println!("Not Found error occurred: {:?}", msg);
                msg.into()
            }
            MyError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(msg) | MyError::ActixError(msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
            MyError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::DBError(err.to_string())
    }
}
