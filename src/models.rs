use url::Url;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use js_sys::{Promise};
use rmp_serde::{Deserializer, Serializer};
use crate::twoface::*;
use crate::utils::*;
use wasm_bindgen_futures::{future_to_promise as ftp, JsFuture};
use web_sys::Request;
use std::convert::TryFrom;

const MAX_POST_CHARS: usize = 1000;

pub async fn new_post(req: Request) -> JsResult {
    let new_post: NewPost = req.into_serde().map_err(|e| e.describe(External{
        status: http::StatusCode::BAD_REQUEST,
        msg: "Your post was malformed".into(),
    }))
    .map_err(|tfe| {
        let v: JsValue = tfe.into();
        v
    })?;
    let post = Post::try_from(new_post)?;
    post.put_first().await.map(|_|JsValue::null()).map_err(|tfe| tfe.into())
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    /// All posts contain some text the user wrote.
    pub text: String,
    /// Posts can optionally link to something.
    pub link: Option<Url>,
    /// User that created this post
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct NewPost {
    /// All posts contain some text the user wrote.
    pub text: String,
    /// Posts can optionally link to something.
    pub link: Option<String>,
    /// User that created this post
    pub user_id: String,
}

impl TryFrom<NewPost> for Post {

    type Error = String;
    
    fn try_from(new_post: NewPost) -> Result<Self, Self::Error> {
        if new_post.text.len() > MAX_POST_CHARS {
            return Err(format!("Posts can only have {} characters, but yours has {}", MAX_POST_CHARS, new_post.text.len()));
        }
        let user_id = Uuid::parse_str(&new_post.user_id).map_err(|_|format!("{} is an invalid user ID", new_post.user_id))?;
        let link = match new_post.link.map(|s|Url::parse(&s)) {
            Some(Err(_)) => return Err("The URL is invalid".to_owned()),
            None => None,
            Some(Ok(u)) => Some(u)
        };
        Ok(Self{
            text: new_post.text,
            link,
            user_id,
        })
    }
}

impl Post {

    pub async fn put_first(self) -> Fallible<()> {
        let key = self.user_id.to_string();
        // let mut val = all_posts_by_user(self.user_id).await?;
        let val = vec![self];
        let mut val_bytes = Vec::new();
        val.serialize(&mut Serializer::new(&mut val_bytes)).map_err(|e| e.describe(External{
            status: http::StatusCode::BAD_REQUEST,
            msg: "Invalid post".into(),
        }))?;
        JsFuture::from(PostsNs::put(&key, &val_bytes)).await.map_err(|e| Error::new(
            anyhow!("{:?}", e),
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "Post unsuccessful, please try again later"
        ))?;

        Ok(())
    }
}

// async fn all_posts_by_user(user_id: Uuid) -> Fallible<Vec<Post>> {

//     let promise = PostsNs::get(&user_id.to_string(), "arrayBuffer");
//     let val = JsFuture::from(promise)
//         .await
//         .map_err(|e| Error::new(
//             anyhow!("{:?}", e),
//             http::StatusCode::INTERNAL_SERVER_ERROR,
//             "couldn't load posts from database"
//         ))?;
//     let typebuf: js_sys::Uint8Array = js_sys::Uint8Array::new(&val);
//     let mut body = vec![0; typebuf.length() as usize];
//     typebuf.copy_to(&mut body[..]);

//     let posts: Vec<Post> = rmp_serde::from_read_ref(&body).map_err(|e| Error::new(
//         e,
//         http::StatusCode::INTERNAL_SERVER_ERROR,
//         "couldn't load posts from database",
//     ))?;
//     Ok(posts)
// }

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
    fn put(key: &str, val: &[u8]) -> Promise;

    #[wasm_bindgen(static_method_of = PostsNs)]
    fn delete(key: &str) -> Promise;
}   