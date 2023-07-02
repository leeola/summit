use axum::response::{Html, IntoResponse, Response};
use hyper::StatusCode;
use pulldown_cmark::Parser;
use sailfish::{
    runtime::{Buffer, Render},
    RenderError, TemplateOnce,
};
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

#[derive(Debug, Clone)]
pub struct MarkdownHtml(pub String);
impl From<String> for MarkdownHtml {
    fn from(s: String) -> Self {
        Self(s)
    }
}
impl Render for MarkdownHtml {
    fn render(&self, b: &mut Buffer) -> Result<(), RenderError> {
        let parser = Parser::new(&self.0);
        // TODO: Reduce allocation here. As far as i can tell this is blocked due to
        // cmark requiring a &mut String, and `Buffer` being only fmt::Write. Cmark has a pull
        // request which may help this?
        let mut str_buf = String::with_capacity(self.0.capacity());
        pulldown_cmark::html::push_html(&mut str_buf, parser);
        str_buf.render(b)
    }
}
