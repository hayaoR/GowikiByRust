use crate::page::{load_page};
use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
    extract::{Path},
    response::{IntoResponse, Html},
};
use tera::Context;
use crate::templates::TEMPLATES;
use std::convert::Infallible;
use tracing::instrument;


#[instrument]
pub async fn view(Path(title): Path<String>) -> impl IntoResponse {
    tracing::info!("called view");
    Title(title)
}

struct Title(String);

impl IntoResponse for Title {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let page = match load_page(&self.0) {
            Ok(p) => p,
            Err(err) => return response_err(err),
        };
        tracing::info!("title {}", &page.title);
        tracing::info!("body {}", &String::from_utf8(page.body.clone()).unwrap());
        let mut context = Context::new();
        context.insert("title", &page.title);
        context.insert("body", &String::from_utf8(page.body).unwrap());
        match TEMPLATES.render("view.html", &context) {
            Ok(html) => Html(html).into_response(),
            Err(err) => response_err(err),
        }
    }
}

fn response_err(err: impl std::error::Error) -> Response<Full<Bytes>> {
    Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!(
                    "Failed to render template. Error: {}",
                    err
                )))
                .unwrap()
}