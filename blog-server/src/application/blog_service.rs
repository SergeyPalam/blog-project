
use actix_web::HttpMessage;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use actix_web::{FromRequest, HttpRequest, dev::Payload};

use std::sync::Arc;
use std::future::{ready, Ready};

use crate::domain::error::AppError;
use crate::domain::post::Post;
use crate::data::{post_repository::PostRepository};
use crate::infrastructure::jwt::{JwtService, Claims};
use tracing::warn;

#[derive(Default, Debug)]
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
pub struct NewPost {
    title: String,
    content: String,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct PostInfo {
    pub content: String,
    pub title: String,
    pub author_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Post> for PostInfo {
    fn from(post: Post) -> Self {
        Self{
            content: post.content,
            title: post.title,
            author_id: post.author_id,
            created_at: post.created_at.to_rfc3339(),
            updated_at: post.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct PostResp {
    offset: i64,
    limit: i64,
    posts: Vec<PostInfo>, 
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

    pub async fn create_post(&self, auth_user: AuthUser, new_post: NewPost) -> Result<PostInfo, AppError>{
        let post_id = self.post_repo.next_post_id().await?;
        let post = Post::create(post_id, new_post.title, new_post.content, auth_user.id);

        self.post_repo.add_new_post(&post).await?;
        Ok(PostInfo::from(post))
    }

    pub async fn get_post(&self, post_id: i64) -> Result<PostInfo, AppError>{
        let post = self.post_repo.get_post(post_id).await?;
        Ok(PostInfo::from(post))
    }

    pub async fn update_post(&self, auth_user: AuthUser, post_id: i64, new_post: NewPost) -> Result<PostInfo, AppError>{
        let author_id = self.post_repo.get_post_author_id(post_id).await?;
        if author_id != auth_user.id {
            warn!("Attempt to update post: {post_id} by user: {:?}", auth_user);
            return Err(AppError::Unauthorized("No permission for update".to_string()));
        }

        let post = self.post_repo.update_post(post_id, new_post.title, new_post.content).await?;
        Ok(PostInfo::from(post))
    }

    pub async fn delete_post(&self, auth_user: AuthUser, post_id: i64) -> Result<(), AppError>{
        let author_id = self.post_repo.get_post_author_id(post_id).await?;
        if author_id != auth_user.id {
            warn!("Attempt to delete post: {post_id} by user: {:?}", auth_user);
            return Err(AppError::Unauthorized("No permission for delete".to_string()));
        }
        self.post_repo.delete_post(post_id).await
    }

    pub async fn get_posts(&self, query: PaginationQuery) -> Result<PostResp, AppError>{
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(10);
        
        let posts = self.post_repo.get_posts(offset, limit).await?;
        let posts_info: Vec<PostInfo> = posts.into_iter().map(|post|{
            PostInfo::from(post)
        }).collect();

        Ok(PostResp{
            offset,
            limit,
            posts: posts_info,
        })
    }
}