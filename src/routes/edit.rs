use axum::{
    extract::{Path},
    response::{IntoResponse, Html},
};
use tera::Context;
use tracing::instrument;

use crate::templates::TEMPLATES;
use crate::page::{load_page};
use crate::handle::handle_error;


#[instrument]
pub async fn edit(Path(title): Path<String>) -> Result<impl IntoResponse, impl IntoResponse> {
    let page = match load_page(&title){
        Ok(page) => page,
        Err(err) => return Err(handle_error(err)),
    };

    tracing::info!("title {}", &page.title);
    tracing::info!("body {}", &String::from_utf8(page.body.clone()).unwrap());
    let mut context = Context::new();
    context.insert("title", &page.title);
    context.insert("body", &String::from_utf8(page.body).unwrap());
    match TEMPLATES.render("edit.html", &context) {
        Ok(html) => Ok(Html(html)),
        Err(err) => Err(handle_error(err)),
    }
}