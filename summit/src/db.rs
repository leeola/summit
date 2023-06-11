use async_trait::async_trait;
use clap::Parser;
use std::sync::Arc;

#[derive(Parser, Debug, Default, Clone)]
pub struct DbConfig {}
impl DbConfig {
    /// Return a [`Db`] implementation based on this configuration.
    #[allow(unreachable_code)]
    pub fn init(&self) -> Arc<dyn Db> {
        #[cfg(feature = "dev")]
        return Arc::new(crate::dev::db::DevDb::default());
        unimplemented!("no non-dev Dbs yet");
    }
}

pub type Result<T, E = DbError> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
#[async_trait]
pub trait Db: Send + Sync {
    async fn posts(&self) -> Result<Vec<Post>>;
}
#[derive(Debug, Clone)]
pub struct Post {
    pub title: String,
}
