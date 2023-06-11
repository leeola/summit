use std::sync::Arc;

use crate::db::{Db, Post};
use axum::extract::State;
use sailfish::TemplateOnce;
use tracing::info;

#[derive(TemplateOnce)]
#[template(path = "community.stpl")]
pub struct Template {
    pub title: String,
    pub posts: Vec<Post>,
}

pub async fn handler(State(db): State<Arc<dyn Db>>) -> String {
    info!("community");

    let posts = db.posts().await.unwrap();
    let ctx = Template {
        title: "Some Title".into(),
        posts,
    };

    ctx.render_once().unwrap()
}
