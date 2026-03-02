//! # blog-client
//! Библиотека для работы с сервером ведения блога

#![warn(missing_docs)]

/// Модуль для работы с ошибками запросов
pub mod error;

/// Модуль для формирования запросов по gRPC
pub mod grpc_client;

/// Модуль для формирования запросов по http
pub mod http_client;

/// Определение pod-типов для формирования Request/Response
pub mod pod;
