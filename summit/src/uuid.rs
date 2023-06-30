//! A general implementation of [`Uuid`] and wrapper types like [`RequestId`] and [`UserId`].
use self::base32_nowords::BASE32_NOWORDS;
use compact_str::CompactString;
use std::fmt;

pub mod base32_nowords;

/// A general purpose centralized uuid, currently using UUIDv7, and encoding itself with  
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Uuid(uuid7::Uuid);
impl Uuid {
    pub fn new() -> Self {
        Self(uuid7::uuid7())
    }
    fn encode(&self) -> CompactString {
        // NOTE: We could use a combination of:
        // - https://docs.rs/data-encoding/latest/data_encoding/struct.Encoding.html#method.encode_mut
        // - https://docs.rs/compact_str/latest/compact_str/struct.CompactString.html#method.as_mut_bytes
        // to reduce this needless alloc, in theory?
        // Ideally it would instead use:
        // - https://docs.rs/compact_str/latest/compact_str/struct.CompactString.html#method.as_mut_str
        // but there's no method on the side of data_encoding to mutate a `&mut str`.
        //
        // Ultimately i'm not doing this because of a needless unsafe. Though i'm not clear if is
        // inlined anyway.. need to check, because i'm curious.
        BASE32_NOWORDS.encode(self.0.as_ref()).into()
    }
}
impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.encode())
    }
}

macro_rules! uuid_impl {
    {
        $(#[$doc:meta])*
        pub struct $name:ident;
    } => {
        $(#[$doc])*
        #[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(pub Uuid);
        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new())
            }
        }
        impl std::ops::Deref for $name {
            type Target = Uuid;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

uuid_impl! {
    pub struct RequestId;
}
uuid_impl! {
    pub struct UserId;
}
