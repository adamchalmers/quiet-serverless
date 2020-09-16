use crate::console_logf;
use crate::twoface::*;
use crate::utils::*;
use http::StatusCode;
use js_sys::Promise;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use url::Url;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

const MAX_POST_CHARS: usize = 1000;

pub async fn new_post(req: Request) -> Result<Response, Response> {
    let json_f = req.json().map_err(|e| {
        Error {
            internal: format!("error getting json future: {:?}", e),
            external_msg: "couldn't get JSON from request".to_owned(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    })?;
    let json = JsFuture::from(json_f).await.map_err(|e| {
        Error {
            internal: format!("error awaiting json: {:?}", e),
            external_msg: "Error awaiting JSON".to_owned(),
            status: StatusCode::BAD_REQUEST,
        }
        .into_response()
    })?;
    let new_post: NewPost = json.into_serde().map_err(|e| {
        Error {
            internal: format!("error parsing post: {:?}", e),
            external_msg: "Your post was malformed".to_owned(),
            status: StatusCode::BAD_REQUEST,
        }
        .into_response()
    })?;
    let post = Post::try_from(new_post).map_err(|e| {
        Error {
            internal: e.clone(),
            external_msg: e,
            status: StatusCode::BAD_REQUEST,
        }
        .into_response()
    })?;
    post.put_first().await.map_err(|e| e.into_response())?;
    console_logf!("Successfully made new post");
    Ok(success_response("you made a post"))
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
            return Err(format!(
                "Posts can only have {} characters, but yours has {}",
                MAX_POST_CHARS,
                new_post.text.len()
            ));
        }
        let user_id = Uuid::parse_str(&new_post.user_id)
            .map_err(|_| format!("{} is an invalid user ID", new_post.user_id))?;
        let link = match new_post.link.map(|s| Url::parse(&s)) {
            Some(Err(_)) => return Err("The URL is invalid".to_owned()),
            None => None,
            Some(Ok(u)) => Some(u),
        };
        Ok(Self {
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
        val.serialize(&mut Serializer::new(&mut val_bytes))
            .map_err(|e| Error {
                internal: e.to_string(),
                status: http::StatusCode::BAD_REQUEST,
                external_msg: "Invalid post".into(),
            })?;
        JsFuture::from(PostsNs::put(&key, &val_bytes))
            .await
            .map_err(|e| Error {
                internal: format!("{:?}", e),
                status: http::StatusCode::INTERNAL_SERVER_ERROR,
                external_msg: "Post unsuccessful, please try again later".to_owned(),
            })?;
        Ok(())
    }
}

pub async fn all_posts_by_user(user_id: Uuid) -> Fallible<Vec<Post>> {
    let promise = PostsNs::get(&user_id.to_string(), "arrayBuffer");
    let val = JsFuture::from(promise).await.map_err(|e| Error {
        internal: format!("{:?}", e),
        status: StatusCode::INTERNAL_SERVER_ERROR,
        external_msg: "couldn't load posts from database".to_owned(),
    })?;
    let typebuf: js_sys::Uint8Array = js_sys::Uint8Array::new(&val);
    let mut body = vec![0; typebuf.length() as usize];
    typebuf.copy_to(&mut body[..]);

    let posts: Vec<Post> = rmp_serde::from_read_ref(&body).map_err(|e| Error {
        internal: format!("{:?}", e),
        status: StatusCode::INTERNAL_SERVER_ERROR,
        external_msg: "couldn't load posts from database".to_owned(),
    })?;
    Ok(posts)
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
    fn put(key: &str, val: &[u8]) -> Promise;

    #[wasm_bindgen(static_method_of = PostsNs)]
    fn delete(key: &str) -> Promise;
}
