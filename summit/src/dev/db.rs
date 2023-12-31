use crate::{
    date_time::DateTime,
    db::{CreatePost, Db, Post, Result},
};
use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct DevDb(RwLock<Inner>);
#[derive(Debug, Default)]
struct Inner {
    posts: Vec<Post>,
}
#[async_trait]
impl Db for DevDb {
    async fn posts(&self) -> Result<Vec<Post>> {
        let db = self.0.read().map_err(|_| anyhow!("lock error"))?;
        Ok(db.posts.iter().cloned().rev().take(100).collect())
    }
    async fn create_post(&self, create_post: CreatePost) -> Result<Post> {
        let CreatePost {
            author,
            title,
            body,
        } = create_post;
        let post = Post {
            // id: "foo".into(),
            created_on: DateTime::now(),
            author,
            title,
            body,
        };
        {
            let mut db = self.0.write().map_err(|_| anyhow!("lock error"))?;
            db.posts.push(post.clone());
        }
        Ok(post)
    }
}
