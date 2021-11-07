use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{header::HeaderMap, StatusCode},
};
use regex::Regex;

pub struct ValidTitle;

#[async_trait]
impl<B> FromRequest<B> for ValidTitle
where
    B: Send,
{
    type Rejection = (StatusCode, HeaderMap, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let re = Regex::new(r"^/[a-zA-Z0-9]+$").unwrap();

        let path = req.uri().path();
        let title = path
            .trim_start_matches('/')
            .trim_start_matches(|c| c != '/');

        if !re.is_match(title) {
            return Err((
                StatusCode::NOT_FOUND,
                HeaderMap::new(),
                "invalid Page Title".to_string(),
            ));
        }

        Ok(Self)
    }
}
