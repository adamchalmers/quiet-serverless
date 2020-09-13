use crate::utils::*;
use crate::console_logf;
use cfg_if::cfg_if;
use handlebars::Handlebars;
use http::StatusCode;
use js_sys::{Array, Promise};
use url::Url;
use crate::templates::{HBARS, TemplateName};
use wasm_bindgen::prelude::*;
use web_sys::{
    FetchEvent, FormData, Headers, Request, Response, ResponseInit, ServiceWorkerGlobalScope,
};
use std::collections::BTreeMap;

const DEFAULT_TITLE: &str = "quiet.";

pub fn generate_error_response(status: StatusCode, msg: Option<&str>) -> JsResult {
    let status_error_msg = format!(
        "{} {}",
        status.as_u16(),
        status.canonical_reason().unwrap_or("Unknown Error")
    );
    let error_message = match msg {
        Some(v) => v.to_owned(),
        None => status_error_msg.to_owned(),
    };
    let data: BTreeMap<_, _> = [("title", "Error"), ("error_message", &error_message)].iter().cloned().collect();
    let body = HBARS.render(TemplateName::Error.name(), &data).ok_or_js_err_with_msg("failed to render error page")?;

    let headers = Headers::new()?;
    headers.append("content-type", "text/html")?;
    let resp = generate_response(&body, status.as_u16(), &headers)?;
    Ok(JsValue::from(resp))
}

pub fn render_error(status: StatusCode) -> Promise {
    match generate_error_response(status, None) {
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
    let data: BTreeMap<_, _> = [("title", "quiet.")].iter().cloned().collect();
    console_logf!("{}", "Rendering...");
    let body = HBARS.render(TemplateName::Test.name(), &data).ok_or_js_err_with_msg("failed to render homepage")?;
    console_logf!("{}: {}", "Rendered", body);
    let headers = Headers::new()?;
    headers.append("content-type", "text/html")?;
    let resp = generate_response(&body, 200, &headers)?;

    Ok(JsValue::from(resp))
}
