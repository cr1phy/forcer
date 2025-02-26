use thiserror::Error;
use tonic::{Code, Status};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Internal error")]
    InternalError,
    #[error("User already exists")]
    UserFound,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
}

impl From<ServerError> for Status {
    fn from(error: ServerError) -> Self {
        match error {
            ServerError::InternalError => Status::new(Code::Internal, "Internal server error"),
            ServerError::UserFound => Status::new(Code::AlreadyExists, "User already exists"),
            ServerError::UserNotFound => Status::new(Code::NotFound, "User not found"),
            ServerError::InvalidCredentials => Status::new(Code::Unauthenticated, "Invalid credentials"),
            ServerError::Unauthorized => Status::new(Code::PermissionDenied, "Unauthorized"),
            ServerError::InvalidRequest(msg) => Status::new(Code::InvalidArgument, msg),
            ServerError::DatabaseError(err) => Status::new(Code::Internal, format!("Database error: {}", err)),
        }
    }
}