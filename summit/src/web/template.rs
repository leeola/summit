use axum::response::{Html, IntoResponse, Response};
use hyper::StatusCode;
use sailfish::TemplateOnce;
use tracing::error;

/// A template response type, where `T` is a sailfish template.
pub struct Template<T>(pub T);
impl<T> IntoResponse for Template<T>
where
    T: TemplateOnce,
{
    fn into_response(self) -> Response {
        match self.0.render_once() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                error!(?err, "failed to render template");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
        }
    }
}
