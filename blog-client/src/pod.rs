use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct RegisterUserReq {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct LoginUserReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct RegisteredUser {
    pub token: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PostId {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetPostsReq {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PostInfo {
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PostResp {
    pub offset: i64,
    pub limit: i64,
    pub posts: Vec<PostInfo>,
}
