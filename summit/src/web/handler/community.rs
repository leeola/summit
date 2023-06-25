use crate::{
    date_time::{DateTime, TimeZone},
    db::{Author, Post},
    web::template::Template,
    Summit,
};
use axum::extract::State;
use sailfish::TemplateOnce;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, TemplateOnce)]
#[template(path = "page/community.stpl")]
pub struct Community<P: Iterator<Item = CommunityPost>> {
    // pub user: NotLoggedIn,
    pub title: String,
    pub posts: P,
}
#[derive(Debug, TemplateOnce)]
#[template(path = "component/community_post.stpl")]
pub struct CommunityPost {
    pub user_tz: TimeZone,
    pub author: Author,
    pub created_on: DateTime,
    pub title: String,
    pub body: String,
}
impl CommunityPost {
    pub fn new(user_tz: TimeZone, post: Post) -> Self {
        let Post {
            author,
            created_on,
            title,
            body,
        } = post;
        Self {
            user_tz,
            author,
            created_on,
            title,
            body,
        }
    }
}

pub async fn handler(
    State(summit): State<Arc<Summit>>,
) -> Template<Community<impl Iterator<Item = CommunityPost>>> {
    info!("community");

    let user_tz = TimeZone::default();
    let posts = summit.posts().await.unwrap();
    Template(Community {
        title: "Some Title".into(),
        posts: posts
            .into_iter()
            .map(move |post| CommunityPost::new(user_tz, post)),
    })
}
