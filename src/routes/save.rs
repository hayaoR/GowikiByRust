use axum::{
    extract::{Path, Form},
    response::{IntoResponse, Redirect},
};
use tracing::instrument;
use serde::Deserialize;

use crate::page::Page;
use crate::handle::handle_error;

#[derive(Deserialize, Debug)]
pub struct Body {
    pub body: String
}

#[instrument]
pub async fn save(Path(title): Path<String>, form: Form<Body>) -> Result<impl IntoResponse, impl IntoResponse> {
   let body = form.0.body.as_bytes().to_vec(); 
   let mut page = Page::new(title.clone(), body);
   match page.save() {
       Ok(_) => Ok(Redirect::found(format!("/view/{}", title).parse().unwrap())),
       Err(err) => Err(handle_error(err)),
   }
   }
