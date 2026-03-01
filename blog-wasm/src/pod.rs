use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RegisterUserReq {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LoginUserReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RegisteredUser {
    pub token: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostId {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GetPostsReq {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostInfo {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostResp {
    pub offset: i64,
    pub limit: i64,
    pub posts: Vec<PostInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub email: String,
    pub id: i64,
    exp: usize,
}