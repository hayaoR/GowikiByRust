use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use tera::Context;
use tracing::instrument;

use crate::handle::{handle_error, handle_redirect};
use crate::page::load_page;
use crate::templates::TEMPLATES;

#[instrument]
pub async fn view(Path(title): Path<String>) -> Result<impl IntoResponse, impl IntoResponse> {
    let page = match load_page(&title) {
        Ok(page) => page,
        Err(_) => return Err(handle_redirect(&title)),
    };

    tracing::info!("title {}", &page.title);
    tracing::info!("body {}", &String::from_utf8(page.body.clone()).unwrap());
    let mut context = Context::new();
    context.insert("title", &page.title);
    context.insert("body", &String::from_utf8(page.body).unwrap());

    match TEMPLATES.render("view.html", &context) {
        Ok(html) => Ok(Html(html)),
        Err(err) => Err(handle_error(err)),
    }
}
