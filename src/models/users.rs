use crate::console_logf;
use crate::twoface;
use crate::twoface::*;
use crate::utils::*;
use chrono::{offset::Utc, DateTime};
use http::StatusCode;
use js_sys::Promise;
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use url::Url;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

const MAX_USERNAME_LENGTH: usize = 32;

pub async fn new_user_profile(req: Request) -> Result<Response, Response> {
    let json_f = req.json().map_err(|e| {
        Error {
            internal: format!("error getting json future: {:?}", e),
            external: twoface::External {
                msg: "couldn't get JSON from request".to_owned(),
            },
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    })?;
    let json = JsFuture::from(json_f).await.map_err(|e| {
        Error {
            internal: format!("error awaiting json: {:?}", e),
            external: twoface::External {
                msg: "Error awaiting JSON".to_owned(),
            },
            status: StatusCode::BAD_REQUEST,
        }
        .into_response()
    })?;
    let new: NewProfile = json.into_serde().map_err(|e| {
        Error {
            internal: format!("error parsing profile: {:?}", e),
            external: twoface::External {
                msg: "Your profile was malformed".to_owned(),
            },
            status: StatusCode::BAD_REQUEST,
        }
        .into_response()
    })?;
    let profile = Profile::try_from(new).map_err(|e| {
        Error {
            internal: e.clone(),
            external: twoface::External { msg: e },
            status: StatusCode::BAD_REQUEST,
        }
        .into_response()
    })?;
    profile.put().await.map_err(|e| e.into_response())?;
    console_logf!("Successfully made new profile");
    Ok(success_response("profile created", Some("/".to_owned())))
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub username: String,
    pub date_joined: DateTime<Utc>,
    pub id: Uuid,
    pub pic: Url,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewProfile {
    pub username: String,
    pub pic: String,
    pub email: String,
}

impl TryFrom<NewProfile> for Profile {
    type Error = String;

    fn try_from(new: NewProfile) -> Result<Self, Self::Error> {
        if new.username.len() > MAX_USERNAME_LENGTH {
            return Err(format!(
                "Usernames can only have {} characters, but yours has {}",
                MAX_USERNAME_LENGTH,
                new.username.len()
            ));
        }
        let id = Uuid::new_v4();
        guard!(let Ok(pic) = Url::parse(&new.pic) else {
            return Err("Your picture URL is invalid".to_owned());
        });
        let email_regex = regex::Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        if !email_regex.is_match(&new.email) {
            return Err("Your email address is invalid".to_owned());
        }
        Ok(Self {
            username: new.username,
            date_joined: Utc::now(),
            id,
            pic,
            email: new.email,
        })
    }
}

impl Profile {
    async fn put(self) -> Fallible<()> {
        let key = self.id.to_string();
        let mut val_bytes = Vec::new();
        self.serialize(&mut Serializer::new(&mut val_bytes))
            .map_err(|e| Error {
                internal: e.to_string(),
                status: http::StatusCode::BAD_REQUEST,
                external: twoface::External {
                    msg: "Invalid post".into(),
                },
            })?;
        JsFuture::from(UsersNs::put(&key, &val_bytes))
            .await
            .map_err(|e| Error {
                internal: format!("{:?}", e),
                status: http::StatusCode::INTERNAL_SERVER_ERROR,
                external: twoface::External {
                    msg: "Couldn't create user, please try again later".to_owned(),
                },
            })?;
        Ok(())
    }
}

#[wasm_bindgen]
extern "C" {
    type UsersNs;

    #[wasm_bindgen(static_method_of = UsersNs)]
    fn get(key: &str, data_type: &str) -> Promise;

    #[wasm_bindgen(static_method_of = UsersNs)]
    fn put(key: &str, val: &[u8]) -> Promise;

    #[wasm_bindgen(static_method_of = UsersNs)]
    fn delete(key: &str) -> Promise;
}
