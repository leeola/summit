use async_trait::async_trait;
use clap::Parser;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Parser, Debug, Default, Clone)]
pub struct DbConfig {}
impl DbConfig {
    /// Return a [`Db`] implementation based on this configuration.
    #[allow(unreachable_code)]
    pub fn init(&self) -> Box<dyn Db> {
        #[cfg(feature = "dev")]
        return Box::<crate::dev::db::DevDb>::default();
        unimplemented!("no non-dev Dbs yet");
    }
}

pub type Result<T, E = DbError> = std::result::Result<T, E>;
#[derive(Debug, Error)]
pub enum DbError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
#[async_trait]
pub trait Db: Send + Sync + Debug {
    async fn posts(&self) -> Result<Vec<Post>>;
    async fn create_post(&self, create_post: CreatePost) -> Result<Post>;
}
#[derive(Debug, Clone)]
pub struct Post {
    pub title: String,
}
#[derive(Debug, Clone)]
pub struct CreatePost {
    pub title: String,
}
