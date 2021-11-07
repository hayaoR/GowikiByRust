use axum::{
    extract::extractor_middleware,
    routing::{get, post},
    Router,
};
use gowiki::check::ValidTitle;
use gowiki::routes::{edit::edit, save::save, view::view};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/view/:title", get(view))
        .route("/edit/:title", get(edit))
        .route("/save/:title", post(save))
        .layer(extractor_middleware::<ValidTitle>());

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
