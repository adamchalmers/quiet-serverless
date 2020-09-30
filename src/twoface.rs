use crate::console_logf;
use http::StatusCode;
use serde::Serialize;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::{Response, ResponseInit};

pub type Fallible<T> = Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub internal: String,
    pub external: External,
    pub status: StatusCode,
}

#[derive(Debug, Serialize)]
pub struct External {
    pub msg: String,
}

impl Error {
    pub fn into_response(self) -> Response {
        console_logf!("{:?}", self.internal);
        let mut init = ResponseInit::new();
        init.status(self.status.into());
        console_logf!("adam 1");
        let body = serde_json::to_string(&self.external)
            .map_err(|e| console_logf!("Error making response {:?}", e))
            .unwrap();
        let resp = Response::new_with_opt_str_and_init(Some(&body), &init)
            .map_err(|e| console_logf!("Error making response {:?}", e))
            .unwrap();
        console_logf!("adam 2");
        resp
    }
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        console_logf!("{:?}", self);
        JsValue::from_str(&self.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "HTTP {}: {}", self.status, self.external.msg)
    }
}
