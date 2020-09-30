#[macro_use]
extern crate guard;
extern crate cfg_if;
extern crate wasm_bindgen;

mod models;
mod templates;
mod twoface;
mod utils;
mod view;

use crate::utils::*;
use cfg_if::cfg_if;
use futures::FutureExt;
use http::StatusCode;
use js_sys::Promise;
use std::future::Future;
use url::Url;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise as ftp;
use web_sys::{FetchEvent, Response};

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
        let err = twoface::Error {
            internal: format!("method {} not allowed for {}", method, url),
            status: StatusCode::NOT_FOUND,
            external_msg: "Page not found".to_owned(),
        };
        view::render_error(err)
    };

    // Route the request to a handler function
    match path.split("/").nth(1) {
        Some("") => match method.as_ref() {
            "get" => ftp(view::render_home(req)),
            _ => render_404(),
        },
        Some("post") => match method.as_ref() {
            "post" => api_result_to_promise(models::posts::new_post(req)),
            "get" => ftp(view::render_new_post(req)),
            _ => render_404(),
        },
        _ => render_404(),
    }
}

fn api_result_to_promise<F>(f: F) -> Promise
where
    F: 'static + Future<Output = Result<Response, Response>>,
{
    let f = f.map(|result| result.map(JsValue::from).map_err(JsValue::from));
    ftp(f)
}
