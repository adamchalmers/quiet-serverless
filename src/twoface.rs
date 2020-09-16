use crate::console_logf;
use http::StatusCode;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::{Response, ResponseInit};

pub type Fallible<T> = Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub internal: String,
    pub external_msg: String,
    pub status: StatusCode,
}

impl Error {
    pub fn into_response(self) -> Response {
        console_logf!("{:?}", self.internal);
        let mut init = ResponseInit::new();
        init.status(self.status.into());
        Response::new_with_opt_str_and_init(Some(&self.external_msg), &init)
            .map_err(|e| console_logf!("Error making response{:?}", e))
            .unwrap()
    }
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        console_logf!("{:?}", self);
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
        write!(f, "HTTP {}: {}", self.status, self.external_msg)
    }
}
