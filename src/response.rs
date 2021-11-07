use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
};

pub fn response_err(err: impl std::error::Error) -> Response<Full<Bytes>> {
    Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!(
                    "Failed to render template. Error: {}",
                    err
                )))
                .unwrap()
}


