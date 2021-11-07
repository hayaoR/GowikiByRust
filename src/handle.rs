use axum::http::{
    header::{HeaderMap, HeaderValue},
    StatusCode,
};

pub fn handle_error(err: impl std::error::Error) -> (StatusCode, HeaderMap, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        HeaderMap::new(),
        format!("Something went wrong: {}", err),
    )
}

pub fn handle_redirect(title: &str) -> (StatusCode, HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::LOCATION,
        HeaderValue::from_str(&format!("/edit/{}", title)).unwrap(),
    );

    (StatusCode::TEMPORARY_REDIRECT, headers, "".to_string())
}
