use std::fmt;
use wasm_bindgen::prelude::*;

pub type Fallible<T> = Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub internal: anyhow::Error,
    pub external: External,
}

impl Error {
    pub fn new<Internal, UserMessage>(
        internal: Internal,
        status: http::StatusCode,
        msg: UserMessage,
    ) -> Self
    where
        Internal: Into<anyhow::Error>,
        UserMessage: fmt::Display,
    {
        internal.describe(External {
            msg: msg.to_string(),
            status,
        })
    }
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}

#[derive(Debug)]
pub struct External {
    pub status: http::StatusCode,
    pub msg: String,
}

pub trait Describe {
    fn describe(self, external: External) -> Error;
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.external.msg)
    }
}

impl<Internal: Into<anyhow::Error>> Describe for Internal {
    fn describe(self, external: External) -> Error {
        Error {
            internal: self.into(),
            external,
        }
    }
}
