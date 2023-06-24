use crate::date_time::DateTime;
use async_trait::async_trait;
use bytesize::ByteSize;
use clap::Parser;
use compact_str::{format_compact, CompactString};
use sailfish::{
    runtime::{Buffer, Render},
    RenderError,
};
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
    // NIT: Will need to introduce either alternate methods for in-app vs federated,
    // or simply allow creation of arbitrary sources.
    async fn create_post(&self, create_post: CreatePost) -> Result<Post>;
}
#[derive(Debug, Clone)]
pub struct Post {
    // pub id: CompactString,
    pub author: Author,
    pub created_on: DateTime,
    pub title: String,
    pub body: String,
}
#[derive(Debug, Clone)]
pub struct CreatePost {
    pub author: Author,
    pub title: String,
    pub body: String,
}
impl CreatePost {
    /// Render the body size, for logging mostly.
    pub fn body_size(&self) -> String {
        // NIT: Why doesn't u64::from(usize) work? :thinking:
        ByteSize::b(self.body.len() as u64).to_string_as(true)
    }
}
#[derive(Debug, Default, Clone)]
pub struct Author {
    // pub id: CompactString,
    pub fedi_addr: FediAddr,
}
#[derive(Debug, Default, Clone)]
pub struct FediAddr {
    pub user: CompactString,
    pub host: CompactString,
}
impl FediAddr {
    pub fn format(&self) -> CompactString {
        let Self { user, host, .. } = self;
        // NIT: Should this alter behavior if name or host are missing? Probably only useful for
        // edgecases like Default or w/e.
        format_compact!("@{user}@{host}")
    }
}
impl Render for FediAddr {
    fn render(&self, buf: &mut Buffer) -> Result<(), RenderError> {
        buf.push_str("@");
        buf.push_str(self.user.as_str());
        buf.push_str("@");
        buf.push_str(self.host.as_str());
        Ok(())
    }
}
