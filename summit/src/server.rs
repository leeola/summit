use crate::db::{CreatePost, Db, DbError, Post};
use thiserror::Error;
use tracing::debug;

pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
impl From<DbError> for Error {
    fn from(err: DbError) -> Self {
        match err {
            DbError::Other(err) => Self::Other(err),
        }
    }
}

#[derive(Debug)]
pub struct Summit {
    db: Box<dyn Db>,
}
impl Summit {
    pub fn new(db: Box<dyn Db>) -> Self {
        Self { db }
    }
    pub async fn create_post(&self, create_post: CreatePost) -> Result<Post> {
        debug!("creating post");
        let post = self.db.create_post(create_post).await?;
        Ok(post)
    }
    pub async fn posts(&self) -> Result<Vec<Post>> {
        let posts = self.db.posts().await?;
        Ok(posts)
    }
}
