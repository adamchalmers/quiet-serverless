use crate::templates::{TemplateName, HBARS};
use crate::twoface;
use crate::utils::*;
use http::StatusCode;
use js_sys::Promise;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;
use web_sys::{Headers, Request, Response, ResponseInit, ServiceWorkerGlobalScope};

lazy_static! {
    static ref BASE: &'static str = TemplateName::Base.name();
}

pub fn generate_error_response(error: twoface::Error) -> JsResult {
    let status = error.external.status;
    let status_error_msg = format!(
        "{} {}",
        status.as_u16(),
        status.canonical_reason().unwrap_or("Unknown Error")
    );
    let data: BTreeMap<_, _> = [("title", "Error"), ("error_message", &error.external.msg)]
        .iter()
        .cloned()
        .collect();
    let body = HBARS
        .render(TemplateName::Error.name(), &data)
        .ok_or_js_err_with_msg("failed to render error page")?;

    let headers = Headers::new()?;
    headers.append("content-type", "text/html")?;
    let resp = generate_response(&body, status.as_u16(), &headers)?;
    Ok(JsValue::from(resp))
}

pub fn render_error(error: twoface::Error) -> Promise {
    match generate_error_response(error) {
        Ok(v) => Promise::resolve(&v),
        Err(e) => Promise::reject(&e),
    }
}

fn generate_response(body: &str, status: u16, headers: &Headers) -> Result<Response, JsValue> {
    let mut init = ResponseInit::new();
    init.status(status);
    init.headers(&JsValue::from(headers));
    Response::new_with_opt_str_and_init(Some(body), &init)
}

pub async fn render_home(request: Request) -> JsResult {
    let data: BTreeMap<_, _> = [("title", "quiet."), ("parent", *BASE)]
        .iter()
        .cloned()
        .collect();
    let body = HBARS
        .render(TemplateName::Home.name(), &data)
        .ok_or_js_err_with_msg("failed to render homepage")?;
    let headers = Headers::new()?;
    headers.append("content-type", "text/html")?;
    let resp = generate_response(&body, 200, &headers)?;

    Ok(JsValue::from(resp))
}

pub async fn render_new_post(request: Request) -> JsResult {
    let data: BTreeMap<_, _> = [("title", "quiet. new post."), ("parent", *BASE)]
        .iter()
        .cloned()
        .collect();
    let body = HBARS
        .render(TemplateName::NewPost.name(), &data)
        .ok_or_js_err_with_msg("failed to render new_post")?;
    let headers = Headers::new()?;
    headers.append("content-type", "text/html")?;
    let resp = generate_response(&body, 200, &headers)?;

    Ok(JsValue::from(resp))
}
