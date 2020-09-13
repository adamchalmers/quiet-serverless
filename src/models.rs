use url::Url;
use uuid::Uuid;
pub struct Post {
    pub link: Url,
    pub text: String,
    pub user_id: Uuid,
}
