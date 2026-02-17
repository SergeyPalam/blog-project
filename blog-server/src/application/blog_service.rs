
use serde::{Serialize, Deserialize};

use std::sync::Arc;

use crate::domain::error::AppError;
use crate::data::{post_repository::PostRepository, user_repository::UserRepository};



#[derive(Deserialize)]
pub struct CreatePostReq {
    title: String,
    content: String,
    token: String,
}

pub struct BlogService {
    user_repo: Arc<UserRepository>,
    post_repo: Arc<PostRepository>,
}

impl BlogService {
    pub fn new(user_repo: Arc<UserRepository>, post_repo: Arc<PostRepository>) -> Self{
        BlogService{
            user_repo,
            post_repo,
        }
    }

    pub fn create_post(author_id: i64, title: String, content: String) -> Result<(), AppError>{
        todo!();
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