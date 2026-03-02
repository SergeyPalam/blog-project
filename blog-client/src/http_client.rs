use reqwest::Client;

use crate::error::ClientError;
use crate::pod::*;

/// Клиент для взаимодействия с сервером по протоколу http
/// Пример:
/// ```rust
/// use blog_client::grpc_client::HttpClient;
/// use blog_client::error::ClientError;
/// use blog_client::pod;
/// 
/// #[tokio::main]
/// async fn main() {
///     let http_client = HttpClient::new("http://127.0.0.1:3000");
///     let reg_req = pod::RegisterUserReq{
///         username,
///         email,
///         password: pass,
///     };
///     let reg_resp = http_client.register(reg_req).await.unwrap();
/// }
/// ``` 
pub struct HttpClient {
    client: Client,
    addr: String,
}

impl HttpClient {
    /// Создание нового HttpClient
    pub fn new(addr: &str) -> Self {
        Self {
            client: Client::new(),
            addr: format!("{addr}/api"),
        }
    }

    /// Регистрация нового пользователя
    pub async fn register(&self, reg_req: RegisterUserReq) -> Result<RegisteredUser, ClientError> {
        let url = format!("{}/auth/register", self.addr);
        let resp = self
            .client
            .post(url)
            .json(&reg_req)
            .send()
            .await?
            .error_for_status()?;

        let reg_user = resp.json::<RegisteredUser>().await?;
        Ok(reg_user)
    }

    /// Вход зарегистрированного пользователя
    pub async fn login(&self, log_req: LoginUserReq) -> Result<RegisteredUser, ClientError> {
        let url = format!("{}/auth/login", self.addr);
        let resp = self
            .client
            .post(url)
            .json(&log_req)
            .send()
            .await?
            .error_for_status()?;

        let reg_user = resp.json::<RegisteredUser>().await?;
        Ok(reg_user)
    }

    /// Создание нового поста (Использует токен, полученный при авторизации)
    pub async fn create_post(
        &self,
        token: &str,
        new_post_req: NewPost,
    ) -> Result<PostInfo, ClientError> {
        let url = format!("{}/posts", self.addr);
        let resp = self
            .client
            .post(url)
            .json(&new_post_req)
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?;

        let post_info = resp.json::<PostInfo>().await?;
        Ok(post_info)
    }

    /// Обновление поста (Использует токен, полученный при авторизации)
    pub async fn update_post(
        &self,
        token: &str,
        post_id: PostId,
        update_post: UpdatePost,
    ) -> Result<PostInfo, ClientError> {
        let url = format!("{}/posts/{}", self.addr, post_id.id);
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

    /// Удаление поста (Использует токен, полученный при авторизации)
    pub async fn delete_post(&self, token: &str, post_id: PostId) -> Result<(), ClientError> {
        let url = format!("{}/posts/{}", self.addr, post_id.id);
        self.client
            .delete(url)
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Получение поста
    pub async fn get_post(&self, post_id: PostId) -> Result<PostInfo, ClientError> {
        let url = format!("{}/posts/{}", self.addr, post_id.id);
        let resp = self.client.get(url).send().await?.error_for_status()?;

        let post_info = resp.json::<PostInfo>().await?;
        Ok(post_info)
    }

    /// Получение списка постов
    pub async fn get_posts(&self, offset: i64, limit: i64) -> Result<PostResp, ClientError> {
        let query = GetPostsReq {
            offset: Some(offset),
            limit: Some(limit),
        };
        let url = format!("{}/posts", self.addr);
        let resp = self
            .client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?;

        let post_info = resp.json::<PostResp>().await?;
        Ok(post_info)
    }
}
