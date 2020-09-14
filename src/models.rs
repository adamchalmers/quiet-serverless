use url::Url;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use js_sys::{Promise};

pub struct Post {
    pub link: Url,
    pub text: String,
    pub user_id: Uuid,
}


// The Cloudflare Workers environment will bind your Workers KV namespaces to
// the name "PostsNs". This is configured in `wrangler.toml`. When your worker
// is run on the Cloudflare edge, there'll be functions called PostsNs.get,
// PostsNs.put and PostsNs.delete, in the top-level JS namespace. This `extern`
// block just tells Rust that those functions will be there at runtime.
#[wasm_bindgen]
extern "C" {
    type PostsNs;

    #[wasm_bindgen(static_method_of = PostsNs)]
    fn get(key: &str, data_type: &str) -> Promise;

    #[wasm_bindgen(static_method_of = PostsNs)]
    fn put(key: &str, val: &str) -> Promise;

    #[wasm_bindgen(static_method_of = PostsNs)]
    fn delete(key: &str) -> Promise;
}   