use crate::db::{CreatePost, Db, Post, Result};
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
        let db = self.0.read().unwrap();
        Ok(db.posts.clone())
    }
    async fn create_post(&self, create_post: CreatePost) -> Result<Post> {
        let post = Post {
            title: create_post.title,
        };
        {
            let mut db = self.0.write().unwrap();
            db.posts.push(post.clone());
        }
        Ok(post)
    }
}
