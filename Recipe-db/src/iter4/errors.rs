use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;


#[derive(Debug, Serialize)]
pub enum RecipeError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}
impl RecipeError {
    fn error_response(&self) -> String {
        match self {
            RecipeError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            RecipeError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            RecipeError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for RecipeError {
    fn status_code(&self) -> StatusCode {
        match self {
            RecipeError::DBError(_msg) | RecipeError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            RecipeError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for RecipeError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From <actix_web::error::Error> for RecipeError 
    {
        fn from (err:actix_web::error::Error) -> Self {
            RecipeError::ActixError(err.to_string())
        }
    }

impl From<SQLxError> for RecipeError {
    fn from(err:SQLxError) -> Self {
        RecipeError::DBError(err.to_string())
    }
}