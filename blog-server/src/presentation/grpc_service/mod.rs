pub mod proto;

use actix_web::web;
use proto::*;
use tonic::{Request, Response, Status};

use crate::application::{auth_service, blog_service};
use crate::domain::error::AppError;
use crate::infrastructure::AppState;
use crate::presentation::grpc_service::proto::blog_service_server::BlogService;

impl From<AppError> for Status {
    fn from(value: AppError) -> Self {
        match value {
            AppError::AlreadyExists(reason) => Self::already_exists(reason),
            AppError::UserNotFound(reason) => Self::not_found(reason),
            AppError::PostNotFound(reason) => Self::not_found(reason),
            AppError::Unauthorized(reason) => Self::unauthenticated(reason),
            AppError::InternalError(reason) => Self::internal(reason),
        }
    }
}

impl From<blog_service::PostInfo> for PostInfo {
    fn from(value: blog_service::PostInfo) -> Self {
        Self {
            id: value.id,
            title: value.title,
            content: value.content,
            author_id: value.author_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub struct BlogGrpcService {
    app_state: web::Data<AppState>,
}

impl BlogGrpcService {
    pub fn new(app_state: web::Data<AppState>) -> Self {
        Self { app_state }
    }
}

#[tonic::async_trait]
impl BlogService for BlogGrpcService {
    async fn register(
        &self,
        in_req: Request<RegisterRequest>,
    ) -> Result<Response<RegisteredUser>, Status> {
        let in_req = in_req.into_inner();
        let auth_service = self.app_state.auth_service.clone();
        let mut reg_request = auth_service::RegisterUserReq::default();
        reg_request.username = in_req.username;
        reg_request.email = in_req.email;
        reg_request.password = in_req.password;
        let reg_user = auth_service.register(reg_request).await?;
        Ok(Response::new(RegisteredUser {
            token: reg_user.token,
        }))
    }

    async fn login(
        &self,
        in_req: Request<LoginRequest>,
    ) -> Result<Response<RegisteredUser>, Status> {
        let in_req = in_req.into_inner();
        let auth_service = self.app_state.auth_service.clone();
        let mut log_request = auth_service::LoginUserReq::default();
        log_request.username = in_req.username;
        log_request.password = in_req.password;
        let reg_user = auth_service.login(log_request).await?;
        Ok(Response::new(RegisteredUser {
            token: reg_user.token,
        }))
    }

    async fn create_post(
        &self,
        in_req: Request<CreatePostRequest>,
    ) -> Result<Response<PostInfo>, Status> {
        let in_req = in_req.into_inner();
        
        let Some(reg_user) = in_req.reg_user else {
            return Err(Status::failed_precondition("token not present"));
        };

        let Some(claims) = self.app_state.jwt_service.verify_token(&reg_user.token) else {
            return Err(Status::unauthenticated("Invalid token"));
        };

        let Some(in_new_post) = in_req.new_post else {
            return Err(Status::failed_precondition("new post not present"));
        };

        let blog_service = self.app_state.blog_service.clone();
        let mut auth_user = blog_service::AuthUser::default();
        auth_user.id = claims.id;
        auth_user.username = claims.username;
        auth_user.email = claims.email;

        let mut new_post = blog_service::NewPost::default();
        new_post.title = in_new_post.title;
        new_post.content = in_new_post.content;

        let out_post_info = blog_service.create_post(auth_user, new_post).await?;
        Ok(Response::new(PostInfo::from(out_post_info)))
    }

    async fn get_post(&self, in_req: Request<PostId>) -> Result<Response<PostInfo>, Status> {
        let in_req = in_req.into_inner();
        let blog_service = self.app_state.blog_service.clone();
        let mut post_id = blog_service::PostId::default();
        post_id.id = in_req.id;

        let out_post_info = blog_service.get_post(post_id).await?;
        Ok(Response::new(PostInfo::from(out_post_info)))
    }

    async fn update_post(
        &self,
        in_req: Request<UpdatePostRequest>,
    ) -> Result<Response<PostInfo>, Status> {
        let in_req = in_req.into_inner();
        let Some(reg_user) = in_req.reg_user else {
            return Err(Status::failed_precondition("token not present"));
        };

        let Some(claims) = self.app_state.jwt_service.verify_token(&reg_user.token) else {
            return Err(Status::unauthenticated("Invalid token"));
        };

        let Some(in_update_post) = in_req.update_post else {
            return Err(Status::failed_precondition("new post not present"));
        };

        let Some(in_post_id) = in_req.post_id else {
            return Err(Status::failed_precondition("post id not present"));
        };

        let blog_service = self.app_state.blog_service.clone();
        let mut auth_user = blog_service::AuthUser::default();
        auth_user.id = claims.id;
        auth_user.username = claims.username;
        auth_user.email = claims.email;

        let mut update_post = blog_service::UpdatePost::default();
        update_post.title = in_update_post.title;
        update_post.content = in_update_post.content;

        let mut post_id = blog_service::PostId::default();
        post_id.id = in_post_id.id;
        let out_post_info = blog_service
            .update_post(auth_user, post_id, update_post)
            .await?;
        Ok(Response::new(PostInfo::from(out_post_info)))
    }

    async fn delete_post(
        &self,
        in_req: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        let in_req = in_req.into_inner();
        let Some(reg_user) = in_req.reg_user else {
            return Err(Status::failed_precondition("token not present"));
        };

        let Some(claims) = self.app_state.jwt_service.verify_token(&reg_user.token) else {
            return Err(Status::unauthenticated("Invalid token"));
        };

        let Some(in_post_id) = in_req.post_id else {
            return Err(Status::failed_precondition("post id not present"));
        };

        let blog_service = self.app_state.blog_service.clone();
        let mut auth_user = blog_service::AuthUser::default();
        auth_user.id = claims.id;
        auth_user.username = claims.username;
        auth_user.email = claims.email;

        let mut post_id = blog_service::PostId::default();
        post_id.id = in_post_id.id;
        blog_service.delete_post(auth_user, post_id).await?;
        Ok(Response::new(DeletePostResponse {}))
    }

    async fn get_posts(
        &self,
        in_req: Request<GetPostsReq>,
    ) -> Result<Response<GetPostsResponse>, Status> {
        let in_req = in_req.into_inner();

        let blog_service = self.app_state.blog_service.clone();
        let mut get_posts_req = blog_service::GetPostsReq::default();
        get_posts_req.offset = Some(in_req.offset);
        get_posts_req.limit = Some(in_req.limit);

        let out_post_info = blog_service.get_posts(get_posts_req).await?;

        let mut res = GetPostsResponse::default();
        res.offset = out_post_info.offset;
        res.limit = out_post_info.limit;
        let posts: Vec<PostInfo> = out_post_info
            .posts
            .into_iter()
            .map(|item| PostInfo::from(item))
            .collect();
        res.posts_info = posts;
        Ok(Response::new(res))
    }
}
