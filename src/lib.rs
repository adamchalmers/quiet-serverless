extern crate cfg_if;
extern crate wasm_bindgen;

mod models;
mod templates;
mod utils;
mod view;

#[macro_use]
use crate::utils::*;
use cfg_if::cfg_if;
use http::StatusCode;
use js_sys::{Promise};
use url::Url;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise as ftp};
use web_sys::{
    FetchEvent
};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}



#[wasm_bindgen]
pub fn main(event: FetchEvent) -> Promise {
    let req = event.request();
    let url = match Url::parse(&req.url()).ok_or_js_err() {
        Ok(v) => v,
        Err(e) => return Promise::reject(&e),
    };
    let path = url.path().to_lowercase();
    let method = req.method().to_lowercase();
    let not_allowed = || view::render_error(StatusCode::METHOD_NOT_ALLOWED);
    console_logf!("{}", "Serving...");
  
    match path.split("/").nth(1) {
        Some("") => match method.as_ref() {
            "get" => ftp(view::render_home(req)),
            _ => not_allowed(),
        },
        _ => view::render_error(StatusCode::NOT_FOUND),
    }
}
