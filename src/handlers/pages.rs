use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "chat.html")]
struct IndexTemplate;

pub async fn index() -> impl IntoResponse {
    IndexTemplate
}
