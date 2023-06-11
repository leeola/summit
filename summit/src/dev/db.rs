use crate::db::{Db, Post, Result};
use async_trait::async_trait;

#[derive(Debug, Default, Clone)]
pub struct DevDb {
    posts: Vec<Post>,
}
#[async_trait]
impl Db for DevDb {
    async fn posts(&self) -> Result<Vec<Post>> {
        Ok(vec![])
    }
}
