use serde::Serialize;

use super::MessageResult;

pub trait ToString {
    fn to_string(&self) -> MessageResult<String>
    where
        Self: Sized;
}

impl<T: Serialize + Sized> ToString for T {
    fn to_string(&self) -> MessageResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}
