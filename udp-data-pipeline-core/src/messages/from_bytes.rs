use serde::Deserialize;

use super::MessageResult;

pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> MessageResult<Self>
    where
        Self: Sized;
}

impl<T: for<'a> Deserialize<'a>> FromBytes for T {
    fn from_bytes(bytes: &[u8]) -> MessageResult<Self> {
        Ok(serde_json::from_slice(bytes)?)
    }
}
