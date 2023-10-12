use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl EzyTutorError {
    fn error_message(&self) -> String {
        match self {
            EzyTutorError::DBError(msg) => {
                println!("Database error occured : {:?}", msg);
                "Database error".into()
            }

            EzyTutorError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            EzyTutorError::NotFound(msg) => {
                println!("Not foundd error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}


impl error::ResponseError for EzyTutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorError::DBError(msg) | EzyTutorError::ActixError(msg) => StatusCode::INTERNAL_SERVER_ERROR,
            EzyTutorError::NotFound(msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_message = self.error_message(); // Use error_message method to get the error message
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: error_message, // Assign error_message to the field
        })
    }
}




impl fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}


impl From<actix_web::error::Error> for EzyTutorError {
    fn from(err: actix_web::error::Error) -> Self {
        EzyTutorError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for EzyTutorError {
    fn from(err: SQLxError) -> Self {
        EzyTutorError::DBError(err.to_string())
    }
}