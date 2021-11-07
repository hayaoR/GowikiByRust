use axum::{
    http::{StatusCode},
};

pub fn handle_error(err: impl std::error::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", err), 
    )
} 