pub mod pod;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use web_sys::Storage;
use gloo_net::http::Request;
use jsonwebtoken::dangerous;
use serde_json::json;

const TOKEN_KEY: &str = "blog_token";

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct BlogApp {
    server_addr: String,
    token: Option<String>,
}

#[wasm_bindgen]
impl BlogApp {
    #[wasm_bindgen(constructor)]
    pub fn new(server_addr: String) -> Result<Self, JsValue> {
        let token = match Self::load_token() {
            Ok(val) => Some(val),
            Err(_) => {
                None
            }
        };

        let blog_app = BlogApp {
            server_addr: format!("{server_addr}/api"),
            token,
        };
        Ok(blog_app)
    }

    fn get_local_storage() -> Result<Storage, JsValue> {
        let Some(window) = web_sys::window() else {
            return Err(JsValue::from_str("Can't get browser window"));
        };

        let Some(storage) = window.local_storage()? else {
            return Err(JsValue::from_str("Not access to local storage"));
        };
        Ok(storage)
    }

    fn save_token_to_storage(token: &str) -> Result<(), JsValue> {
        let storage = Self::get_local_storage()?;
        storage.set_item(TOKEN_KEY, token)
    }

    fn load_token() -> Result<String, JsValue> {
        let storage = Self::get_local_storage()?;
        let Some(token) = storage.get_item(TOKEN_KEY)? else {
            return Err(JsValue::from_str("Can't get token from storage"));
        };
        Ok(token)
    }

    fn remove_token() -> Result<(), JsValue> {
        let storage = Self::get_local_storage()?;
        storage.remove_item(TOKEN_KEY)
    }

    fn make_js_result(json_str: &str) -> Result<JsValue, JsValue> {
        js_sys::JSON::parse(&json_str)
    }

    fn make_success_response() -> Result<JsValue, JsValue> {
        let val = json!({"success": "true"});
        Self::make_js_result(&val.to_string())
    }

    #[wasm_bindgen]
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    #[wasm_bindgen]
    pub fn get_current_user_id(&self) -> Result<JsValue, JsValue> {
        if let Some(token) = self.token.as_ref() {
            let id = match dangerous::insecure_decode::<pod::Claims>(token) {
                Ok(token_data) => token_data.claims.id,
                Err(e) => {
                    return Err(JsValue::from_str(&e.to_string()))
                }
            };
            
            return Self::make_js_result(&serde_json::json!({"id": id}).to_string());
        }

        Err(JsValue::from_str("User logged out"))
    }

    #[wasm_bindgen]
    pub async fn register(&mut self, username: String, email: String, password: String) -> Result<JsValue, JsValue> {
        let url = format!("{}/auth/register", self.server_addr);
        let req = pod::RegisterUserReq {
            username,
            email,
            password,
        };

        let resp = Request::post(&url)
            .json(&req)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let reg_user = resp.json::<pod::RegisteredUser>().await.map_err(|e| e.to_string())?;
        Self::save_token_to_storage(&reg_user.token)?;
        self.token = Some(reg_user.token);

        Self::make_success_response()
    }

    #[wasm_bindgen]
    pub async fn login(&mut self, username: String, password: String) -> Result<JsValue, JsValue> {
        let url = format!("{}/auth/login", self.server_addr);
        let req = pod::LoginUserReq {
            username,
            password,
        };

        let resp = Request::post(&url)
            .json(&req)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let reg_user = resp.json::<pod::RegisteredUser>().await.map_err(|e| e.to_string())?;
        Self::save_token_to_storage(&reg_user.token)?;
        self.token = Some(reg_user.token);

        Self::make_success_response()
    }

    #[wasm_bindgen]
    pub async fn logout(&mut self) -> Result<JsValue, JsValue> {
        Self::remove_token()?;
        self.token = None;
        Self::make_success_response()
    }

    #[wasm_bindgen]
    pub async fn create_post(&self, title: String, content: String) -> Result<JsValue, JsValue> {
        let Some(token) = self.token.as_ref() else {
            return Err(JsValue::from_str("Auth needed"));
        };

        let url = format!("{}/posts", self.server_addr);
        let req = pod::NewPost {
            title,
            content,
        };

        let _resp = Request::post(&url)
            .header("Authorization", &format!("Bearer {}", token))
            .json(&req)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        
        Self::make_success_response()
    }

    #[wasm_bindgen]
    pub async fn update_post(&self,
        post_id: i64,
        title: Option<String>,
        content: Option<String>) -> Result<JsValue, JsValue> 
    {
        let Some(token) = self.token.as_ref() else {
            return Err(JsValue::from_str("Auth needed"));
        };

        let url = format!("{}/posts/{}", self.server_addr, post_id);
        let req = pod::UpdatePost{
            title,
            content
        };

        let _resp = Request::put(&url)
            .header("Authorization", &format!("Bearer {}", token))
            .json(&req)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Self::make_success_response()
    }

    #[wasm_bindgen]
    pub async fn delete_post(&self, post_id: i64) -> Result<JsValue, JsValue> {
        let Some(token) = self.token.as_ref() else {
            return Err(JsValue::from_str("Auth needed"));
        };

        let url = format!("{}/posts/{}", self.server_addr, post_id);

        let _resp = Request::delete(&url)
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Self::make_success_response()
    }

    #[wasm_bindgen]
    pub async fn get_posts(&self, offset: i64, limit: i64) -> Result<JsValue, JsValue> {
        let url = format!("{}/posts", self.server_addr);
        let slice_params = vec![("offset", offset.to_string()), ("limit", limit.to_string())];

        let resp = Request::get(&url)
            .query(slice_params)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let posts_json_str = resp.text().await.map_err(|e| e.to_string())?;
        Self::make_js_result(&posts_json_str)
    }
}
