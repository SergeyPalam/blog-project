pub mod proto;

use proto::blog_service_client::BlogServiceClient;
use proto::*;

use crate::error::ClientError;
use crate::pod;

impl From<PostInfo> for pod::PostInfo {
    fn from(value: PostInfo) -> Self {
        Self {
            title: value.title,
            content: value.content,
            author_id: value.author_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub struct GrpcClient {
    client: BlogServiceClient<tonic::transport::Channel>,
}

impl GrpcClient {
    pub async fn connect(addr: &str) -> Result<Self, ClientError> {
        let client = match BlogServiceClient::connect(addr.to_string()).await {
            Ok(res) => res,
            Err(e) => {
                return Err(ClientError::NotFound(format!("{e}")));
            }
        };
        Ok(Self { client })
    }

    pub async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<pod::RegisteredUser, ClientError> {
        let response = self
            .client
            .register(RegisterRequest {
                username,
                email,
                password,
            })
            .await?
            .into_inner();

        let reg_user = pod::RegisteredUser {
            token: response.token,
        };
        Ok(reg_user)
    }

    pub async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<pod::RegisteredUser, ClientError> {
        let response = self
            .client
            .login(LoginRequest { username, password })
            .await?
            .into_inner();

        let reg_user = pod::RegisteredUser {
            token: response.token,
        };
        Ok(reg_user)
    }

    pub async fn create_post(
        &mut self,
        token: &str,
        title: String,
        content: String,
    ) -> Result<pod::PostInfo, ClientError> {
        let response = self
            .client
            .create_post(CreatePostRequest {
                reg_user: Some(RegisteredUser {
                    token: token.to_string(),
                }),
                new_post: Some(NewPost { title, content }),
            })
            .await?
            .into_inner();

        Ok(response.into())
    }

    pub async fn update_post(
        &mut self,
        token: &str,
        post_id: i64,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<pod::PostInfo, ClientError> {
        let response = self
            .client
            .update_post(UpdatePostRequest {
                reg_user: Some(RegisteredUser {
                    token: token.to_string(),
                }),
                update_post: Some(UpdatePost { title, content }),
                post_id: Some(PostId { id: post_id }),
            })
            .await?
            .into_inner();

        Ok(response.into())
    }

    pub async fn delete_post(&mut self, token: &str, post_id: i64) -> Result<(), ClientError> {
        let _response = self
            .client
            .delete_post(DeletePostRequest {
                reg_user: Some(RegisteredUser {
                    token: token.to_string(),
                }),
                post_id: Some(PostId { id: post_id }),
            })
            .await?
            .into_inner();

        Ok(())
    }

    pub async fn get_post(&mut self, post_id: i64) -> Result<pod::PostInfo, ClientError> {
        let response = self
            .client
            .get_post(PostId { id: post_id })
            .await?
            .into_inner();

        Ok(response.into())
    }

    pub async fn get_posts(
        &mut self,
        offset: i64,
        limit: i64,
    ) -> Result<pod::PostResp, ClientError> {
        let response = self
            .client
            .get_posts(GetPostsReq { offset, limit })
            .await?
            .into_inner();

        let posts_info: Vec<pod::PostInfo> = response
            .posts_info
            .into_iter()
            .map(|item| pod::PostInfo::from(item))
            .collect();

        Ok(pod::PostResp {
            offset,
            limit,
            posts: posts_info,
        })
    }
}
