use crate::{date_time::TimeZone, db::Post, web::template::Template, Summit};
use axum::extract::State;
use sailfish::TemplateOnce;
use std::sync::Arc;
use tracing::info;

#[derive(TemplateOnce)]
#[template(path = "page/community.stpl")]
pub struct Community {
    // pub user: NotLoggedIn,
    pub user_tz: TimeZone,
    pub title: String,
    pub posts: Vec<Post>,
}

pub async fn handler(State(summit): State<Arc<Summit>>) -> Template<Community> {
    info!("community");

    let posts = summit.posts().await.unwrap();
    Template(Community {
        user_tz: TimeZone::default(),
        title: "Some Title".into(),
        posts,
    })
}
