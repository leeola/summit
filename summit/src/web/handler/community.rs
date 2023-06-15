use crate::{db::Post, server::Summit, web::template::Template};
use axum::extract::State;
use sailfish::TemplateOnce;
use std::sync::Arc;
use tracing::info;

#[derive(TemplateOnce)]
#[template(path = "page/community.stpl")]
pub struct Community {
    pub title: String,
    pub posts: Vec<Post>,
}

pub async fn handler(State(summit): State<Arc<Summit>>) -> Template<Community> {
    info!("community");

    let posts = summit.posts().await.unwrap();
    Template(Community {
        title: "Some Title".into(),
        posts,
    })
}
