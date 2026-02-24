use reqwest::Client;

use crate::error::ClientError;
use crate::pod::*;

pub struct HttpClient {
    client: Client,
    addr: String,
}

impl HttpClient {
    pub fn new(addr: &str) -> Self {
        Self {
            client: Client::new(),
            addr: addr.to_string(),
        }
    }

    pub async fn register(&self, reg_req: RegisterUserReq) -> Result<RegisteredUser, ClientError> {
        let resp = self
            .client
            .post(&self.addr)
            .json(&reg_req)
            .send()
            .await?
            .error_for_status()?;

        let reg_user = resp.json::<RegisteredUser>().await?;
        Ok(reg_user)
    }

    pub async fn login(&self, log_req: LoginUserReq) -> Result<RegisteredUser, ClientError> {
        let resp = self
            .client
            .post(&self.addr)
            .json(&log_req)
            .send()
            .await?
            .error_for_status()?;

        let reg_user = resp.json::<RegisteredUser>().await?;
        Ok(reg_user)
    }

    pub async fn create_post(
        &self,
        token: &str,
        new_post_req: NewPost,
    ) -> Result<PostInfo, ClientError> {
        let resp = self
            .client
            .post(&self.addr)
            .json(&new_post_req)
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?;

        let post_info = resp.json::<PostInfo>().await?;
        Ok(post_info)
    }

    pub async fn update_post(
        &self,
        token: &str,
        post_id: PostId,
        update_post: UpdatePost,
    ) -> Result<PostInfo, ClientError> {
        let url = format!("{}/{}", self.addr, post_id.id);
        let resp = self
            .client
            .put(url)
            .bearer_auth(token)
            .json(&update_post)
            .send()
            .await?
            .error_for_status()?;

        let post_info = resp.json::<PostInfo>().await?;
        Ok(post_info)
    }

    pub async fn delete_post(&self, token: &str, post_id: PostId) -> Result<(), ClientError> {
        let url = format!("{}/{}", self.addr, post_id.id);
        self.client
            .delete(url)
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn get_post(&self, post_id: PostId) -> Result<PostInfo, ClientError> {
        let url = format!("{}/{}", self.addr, post_id.id);
        let resp = self.client.get(url).send().await?.error_for_status()?;

        let post_info = resp.json::<PostInfo>().await?;
        Ok(post_info)
    }

    pub async fn get_posts(&self, offset: i64, limit: i64) -> Result<PostResp, ClientError> {
        let query = GetPostsReq {
            offset: Some(offset),
            limit: Some(limit),
        };
        let resp = self
            .client
            .get(&self.addr)
            .query(&query)
            .send()
            .await?
            .error_for_status()?;

        let post_info = resp.json::<PostResp>().await?;
        Ok(post_info)
    }
}
