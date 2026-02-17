
use actix_web::HttpMessage;
use serde::{Serialize, Deserialize};

use actix_web::{FromRequest, HttpRequest, dev::Payload};

use std::sync::Arc;
use std::future::{ready, Ready};

use crate::domain::error::AppError;
use crate::domain::post::Post;
use crate::data::{post_repository::PostRepository, user_repository::UserRepository};
use crate::infrastructure::jwt::{JwtService, Claims};

#[derive(Default)]
pub struct AuthUser {
    username: String,
    email: String,
    id: i64,
}

impl FromRequest for AuthUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(claims) = req.extensions().get::<Claims>() {
            let mut auth_user = AuthUser::default();
            auth_user.id = claims.id;
            auth_user.username = claims.username.clone();
            auth_user.email = claims.email.clone();
            return ready(Ok(auth_user));
        }
        ready(Err(AppError::InternalError("Invalid type".to_string())))
    }
}

#[derive(Deserialize)]
pub struct CreatePostReq {
    title: String,
    content: String,
}

#[derive(Serialize)]
pub struct CreatedPost {
    title: String,
    content: String,
    token: String,
}

pub struct BlogService {
    post_repo: Arc<PostRepository>,
}

impl BlogService {
    pub fn new(post_repo: Arc<PostRepository>) -> Self{
        BlogService{
            post_repo,
        }
    }

    pub async fn create_post(&self, auth_user: AuthUser, new_post: CreatePostReq) -> Result<String, AppError>{
        let post = Post::new(new_post.title, new_post.content, auth_user.id);
        self.post_repo.add_new_post(post).await
    }

    pub fn update_post_title(author_id: i64, post_id: i64, new_title: String) -> Result<(), AppError>{
        todo!();
    }

    pub fn update_post_content(author_id: i64, post_id: i64, new_content: String) -> Result<(), AppError>{
        todo!();
    }

    pub fn delete_post(author_id: i64, post_id: i64) -> Result<(), AppError>{
        todo!();
    }
    pub fn get_posts(limit: usize, offset: usize) -> Result<String, AppError> {
        todo!();
    }
}