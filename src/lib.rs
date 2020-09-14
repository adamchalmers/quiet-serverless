#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate guard;
extern crate cfg_if;
extern crate wasm_bindgen;

mod models;
mod templates;
mod utils;
mod twoface;
mod view;

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
    let render_404 = || {
        let err = twoface::Error::new(
            anyhow!("method {} not allowed for {}", method, url), 
            StatusCode::NOT_FOUND, 
            "Page not found",
        );
        view::render_error(err)
    };
  
    // Route the request to a handler function
    match path.split("/").nth(1) {
        Some("") => match method.as_ref() {
            "get" => ftp(view::render_home(req)),
            _ => render_404(),
        },
        Some("post") => match method.as_ref() {
            "post" => ftp(models::new_post(req)),
            _ => render_404(),
        },
        _ => render_404(),
    }
}
