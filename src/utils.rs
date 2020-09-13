use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub type JsResult = Result<JsValue, JsValue>;

use js_sys::Error;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

pub trait ToJsResult<T> {
    fn ok_or_js_err(self) -> Result<T, JsValue>;
}

pub trait ToJsResultWithMsg<T> {
    fn ok_or_js_err_with_msg(self, msg: &str) -> Result<T, JsValue>;
}

impl<T> ToJsResult<T> for Option<T> {
    fn ok_or_js_err(self) -> Result<T, JsValue> {
        match self {
            Some(v) => Ok(v),
            None => Err(JsValue::from(Error::new("expected Some but found None"))),
        }
    }
}

impl<T> ToJsResultWithMsg<T> for Option<T> {
    fn ok_or_js_err_with_msg(self, msg: &str) -> Result<T, JsValue> {
        match self {
            Some(v) => Ok(v),
            None => Err(JsValue::from(Error::new(msg))),
        }
    }
}

impl<T, E> ToJsResult<T> for Result<T, E>
where
    E: Display,
{
    fn ok_or_js_err(self) -> Result<T, JsValue> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(JsValue::from(Error::new(&e.to_string()))),
        }
    }
}

impl<T, E> ToJsResultWithMsg<T> for Result<T, E> {
    fn ok_or_js_err_with_msg(self, msg: &str) -> Result<T, JsValue> {
        match self {
            Ok(v) => Ok(v),
            Err(_e) => Err(JsValue::from(Error::new(msg))),
        }
    }
}

#[macro_export]
macro_rules! console_logf {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}