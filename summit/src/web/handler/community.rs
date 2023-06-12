use crate::{db::Post, server::Summit};
use axum::extract::State;
use sailfish::TemplateOnce;
use std::sync::Arc;
use tracing::info;

#[derive(TemplateOnce)]
#[template(path = "community.stpl")]
pub struct Template {
    pub title: String,
    pub posts: Vec<Post>,
}

pub async fn handler(State(summit): State<Arc<Summit>>) -> String {
    info!("community");

    let posts = summit.posts().await.unwrap();
    let ctx = Template {
        title: "Some Title".into(),
        posts,
    };

    ctx.render_once().unwrap()
}
