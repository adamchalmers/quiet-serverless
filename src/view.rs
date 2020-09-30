use crate::models::posts;
use crate::templates::{TemplateName, HBARS};
use crate::twoface;
use crate::utils::*;
use js_sys::Promise;
use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::BTreeMap;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use web_sys::{Headers, Request, Response, ResponseInit};

lazy_static! {
    static ref BASE: &'static str = TemplateName::Base.name();
}

pub fn generate_error_response(error: twoface::Error) -> JsResult {
    let status = error.status;
    let http_error = format!(
        "{} {}",
        status.as_u16(),
        status.canonical_reason().unwrap_or("Unknown Error")
    );
    let data: BTreeMap<_, _> = [
        ("title", "Error"),
        ("error_message", &error.external.msg),
        ("http_error", &http_error),
    ]
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

pub async fn render_home(_: Request) -> JsResult {
    let test_user = Uuid::parse_str("fc53b101-1756-4b8f-b5fe-b71d103e9f20").unwrap();
    let posts = match posts::all_posts_by_user(test_user).await {
        Ok(p) => p,
        Err(e) => return generate_error_response(e),
    };
    #[derive(Serialize)]
    struct Data {
        title: String,
        parent: String,
        posts: Vec<posts::Post>,
        post_list_template: String,
    }
    let data = Data {
        title: "quiet".to_owned(),
        parent: BASE.to_string(),
        posts,
        post_list_template: TemplateName::PostList.name().to_owned(),
    };
    let body = HBARS
        .render(TemplateName::Home.name(), &data)
        .ok_or_js_err_with_msg("failed to render homepage")?;
    let headers = Headers::new()?;
    headers.append("content-type", "text/html")?;
    let resp = generate_response(&body, 200, &headers)?;

    Ok(JsValue::from(resp))
}

pub async fn render_new_post(_: Request) -> JsResult {
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
