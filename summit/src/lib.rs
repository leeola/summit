use crate::db::{CreatePost, Db, DbError, Post};
use date_time::TimeZone;
use std::fmt;
use thiserror::Error;
use tracing::{debug, instrument};

pub mod date_time;
pub mod db;
#[cfg(any(test, feature = "dev"))]
pub mod dev;
pub mod web;

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

pub struct Summit {
    db: Box<dyn Db>,
}
impl Summit {
    pub fn new(db: Box<dyn Db>) -> Self {
        Self { db }
    }
    #[instrument(skip_all, fields(
        // user_id=create_post.author.id,
        ?user_fedi_addr=create_post.author.fedi_addr.format(),
        post_size=create_post.body_size(),
    ))]
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
impl fmt::Debug for Summit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Mostly just a placeholder for eventually lazily recording some maybe useful information
        // about Summit, event counts, connection counts, etc.
        f.debug_struct("Summit").finish()
    }
}

#[derive(Debug, Clone)]
pub struct User {
    // pub id: CompactString,
    pub time_zone: TimeZone,
}
