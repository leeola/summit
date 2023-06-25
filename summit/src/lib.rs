use crate::db::{CreatePost, Db, DbError, Post};
use anyhow::anyhow;
use date_time::TimeZone;
use kanal::{AsyncReceiver, AsyncSender};
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    sync::Mutex,
};
use thiserror::Error;
use tracing::{debug, instrument};
use uuid::{RequestId, UserId};

pub mod date_time;
pub mod db;
#[cfg(any(test, feature = "dev"))]
pub mod dev;
pub mod uuid;
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
    /// CONCURRENCY: Prototype design, this is naive on purpose. I also have no firm design on
    /// propagating and filtering events to users yet, so don't prematurely engineer .. right?
    user_events: Mutex<HashMap<UserId, BTreeMap<RequestId, (AsyncSender<()>, AsyncReceiver<()>)>>>,
}
impl Summit {
    pub fn new(db: Box<dyn Db>) -> Self {
        Self {
            db,
            user_events: Default::default(),
        }
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
    pub async fn open_event_stream(
        &self,
        user_id: UserId,
        req_id: RequestId,
    ) -> Result<AsyncReceiver<()>> {
        let mut user_events = self.user_events.lock().map_err(|err| anyhow!("{err}"))?;
        let (_, receiver) = user_events
            .entry(user_id)
            .or_default()
            .entry(req_id)
            .or_insert_with(|| {
                let (sender, receiver) = kanal::bounded_async::<()>(2);
                (sender, receiver)
            });
        Ok(receiver.clone())
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
