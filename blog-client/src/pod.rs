use serde::{Deserialize, Serialize};

/// pod Запроса регистрации
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RegisterUserReq {
    /// Имя пользователя
    pub username: String,
    /// Почта пользователя
    pub email: String,
    /// Пароль
    pub password: String,
}

/// pod запроса входа
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LoginUserReq {
    /// Имя пользователя
    pub username: String,
    /// Пароль
    pub password: String,
}

/// Ответ на регистрацию или вход пользователя
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RegisteredUser {
    /// Токен для авторизации
    pub token: String,
}

/// pod Создания нового поста
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NewPost {
    /// Заголовок
    pub title: String,
    /// Содержимое
    pub content: String,
}

/// pod обновления поста
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdatePost {
    /// Новый заголовок
    pub title: Option<String>,
    /// Новый контент
    pub content: Option<String>,
}

/// pod Id поста
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostId {
    /// id поста
    pub id: i64,
}

/// pod Запрос списка постов
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GetPostsReq {
    /// Номер страницы
    pub offset: Option<i64>,
    /// Количество постов в странице
    pub limit: Option<i64>,
}

/// pod Информация о посте
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostInfo {
    /// id-поста
    pub id: i64,
    /// Заголовок поста
    pub title: String,
    /// Содержимое поста
    pub content: String,
    /// id автора
    pub author_id: i64,
    /// Дата создание поста UTC rfc 3339
    pub created_at: String,
    /// Дата обновления поста UTC rfc 3339
    pub updated_at: String,
}

/// pod ответ на запрос списка постов
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostResp {
    /// Номер страницы
    pub offset: i64,
    /// Количество постов в странице
    pub limit: i64,
    /// Список постов
    pub posts: Vec<PostInfo>,
}
