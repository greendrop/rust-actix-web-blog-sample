use actix_web::{
    delete, get,
    http::{header::ContentType, StatusCode},
    patch, post, web, HttpResponse, Responder,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::repository;

#[derive(Serialize)]
struct HttpErrorResponse {
    code: String,
    message: String,
}

impl HttpErrorResponse {
    fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    fn not_found() -> Self {
        Self::new("NOT_FOUND", "Not Found")
    }

    fn internal_server_error() -> Self {
        Self::new("INTERNAL_SERVER_ERROR", "Internal Server Error")
    }
}

#[derive(Debug, Display)]
pub enum AppErrorKind {
    #[display(fmt = "internal server error")]
    InternalServerError,

    #[display(fmt = "not found")]
    NotFound,
}

#[derive(Debug, Display)]
#[display(fmt = "{} {}", kind, err)]
pub struct AppError {
    kind: AppErrorKind,
    err: anyhow::Error,
}

impl AppError {
    pub fn internal_server_error(err: anyhow::Error) -> Self {
        Self {
            kind: AppErrorKind::InternalServerError,
            err,
        }
    }

    pub fn not_found() -> Self {
        Self {
            kind: AppErrorKind::NotFound,
            err: anyhow::anyhow!("Not Found"),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> AppError {
        AppError {
            kind: AppErrorKind::InternalServerError,
            err,
        }
    }
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(match self.kind {
                AppErrorKind::InternalServerError => HttpErrorResponse::internal_server_error(),
                AppErrorKind::NotFound => HttpErrorResponse::not_found(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match self.kind {
            AppErrorKind::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorKind::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

#[derive(Serialize)]
struct ArticleIndexResponse {
    id: i32,
    title: String,
    body: String,
}

#[derive(Serialize)]
struct ArticleShowResponse {
    id: i32,
    title: String,
    body: String,
}

#[derive(Deserialize)]
struct ArticleForm {
    title: String,
    body: String,
}

#[derive(Serialize)]
struct CommentIndexResponse {
    id: i32,
    body: String,
}

#[derive(Serialize)]
struct CommentShowResponse {
    id: i32,
    body: String,
}

#[derive(Deserialize)]
struct CommentForm {
    body: String,
}

pub fn notify_error_handler<B>(
    res: actix_web::dev::ServiceResponse<B>,
) -> actix_web::Result<actix_web::middleware::ErrorHandlerResponse<B>> {
    let uuid = sentry::types::Uuid::new_v4();
    let request = res.request();
    let mut event = sentry::protocol::Event {
        event_id: uuid,
        message: Some("Internal Server Error".into()),
        level: sentry::protocol::Level::Error,
        request: Some(sentry::protocol::Request {
            url: format!(
                "{}://{}{}",
                request.connection_info().scheme(),
                request.connection_info().host(),
                request.uri()
            )
            .parse()
            .ok(),
            method: Some(request.method().to_string()),
            headers: request
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
                .collect(),
            ..Default::default()
        }),
        ..Default::default()
    };

    match res.response().error() {
        Some(err) => match err.as_error::<AppError>() {
            Some(err) => {
                event.message = Some(err.err.to_string());
                let backtrace = err.err.backtrace();
                event.stacktrace =
                    sentry::integrations::backtrace::parse_stacktrace(&format!("{backtrace:#}"));
                println!("AppError: {}", err.err);
            }
            None => {}
        },
        None => {}
    }

    sentry::capture_event(event.clone());

    Ok(actix_web::middleware::ErrorHandlerResponse::Response(
        res.map_into_left_body(),
    ))
}

#[get("/")]
async fn hello() -> impl Responder {
    // panic!("Everything is on fire!");
    // sentry::capture_message("Something is not well", sentry::Level::Warning);

    // let uuid = sentry::types::Uuid::new_v4();
    // let event = sentry::protocol::Event {
    //     event_id: uuid,
    //     message: Some("Hello World!".into()),
    //     level: sentry::protocol::Level::Info,
    //     ..Default::default()
    // };

    // sentry::capture_event(event.clone());

    HttpResponse::Ok().body("Hello world!")
}

#[get("/articles")]
async fn articles_index(data: web::Data<super::AppState>) -> Result<HttpResponse, AppError> {
    let dtabase_connection = &data.database_connection;

    let articles_repository = repository::ArticlesRepository::new(dtabase_connection.clone());

    match articles_repository.find_all().await {
        Ok(articles) => {
            let response = articles
                .iter()
                .map(|article| ArticleIndexResponse {
                    id: article.id,
                    title: article.title.clone(),
                    body: article.body.clone(),
                })
                .collect::<Vec<ArticleIndexResponse>>();
            return Ok(HttpResponse::Ok().json(response));
        }
        Err(err) => Err(AppError::internal_server_error(err.into())),
    }
}

#[post("/articles")]
async fn articles_create(
    data: web::Data<super::AppState>,
    article_form: web::Json<ArticleForm>,
) -> Result<HttpResponse, AppError> {
    let article_form = article_form.into_inner();
    let dtabase_connection = &data.database_connection;

    let articles_repository = repository::ArticlesRepository::new(dtabase_connection.clone());

    let form = entity::articles::Model {
        id: 0,
        title: article_form.title,
        body: article_form.body,
    };

    match articles_repository.create(form).await {
        Ok(article) => {
            let response = ArticleShowResponse {
                id: article.id.unwrap(),
                title: article.title.unwrap(),
                body: article.body.unwrap(),
            };
            return Ok(HttpResponse::Created().json(response));
        }
        Err(err) => Err(AppError::internal_server_error(err.into())),
    }
}

#[get("/articles/{id}")]
async fn articles_show(
    data: web::Data<super::AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = id.into_inner();
    let dtabase_connection = &data.database_connection;

    let articles_repository = repository::ArticlesRepository::new(dtabase_connection.clone());

    match articles_repository.find_by_id(id).await {
        Ok(ok) => match ok {
            Some(article) => {
                let response = ArticleShowResponse {
                    id: article.id,
                    title: article.title,
                    body: article.body,
                };
                Ok(HttpResponse::Ok().json(response))
            }
            None => Err(AppError::not_found()),
        },
        Err(err) => Err(AppError::internal_server_error(err.into())),
    }
}

#[patch("/articles/{id}")]
async fn articles_update(
    data: web::Data<super::AppState>,
    id: web::Path<i32>,
    article_form: web::Json<ArticleForm>,
) -> Result<HttpResponse, AppError> {
    let id = id.into_inner();
    let dtabase_connection = &data.database_connection;

    let articles_repository = repository::ArticlesRepository::new(dtabase_connection.clone());

    match articles_repository.find_by_id(id).await {
        Ok(ok) => match ok {
            Some(_) => {
                let article_form = article_form.into_inner();

                let form = entity::articles::Model {
                    id,
                    title: article_form.title,
                    body: article_form.body,
                };

                match articles_repository.update(form).await {
                    Ok(_) => Ok(HttpResponse::NoContent().body("")),
                    Err(err) => Err(AppError::internal_server_error(err.into())),
                }
            }
            None => Err(AppError::not_found()),
        },
        Err(err) => Err(AppError::internal_server_error(err.into())),
    }
}

#[delete("/articles/{id}")]
async fn articles_delete(
    data: web::Data<super::AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = id.into_inner();
    let dtabase_connection = &data.database_connection;

    let articles_repository = repository::ArticlesRepository::new(dtabase_connection.clone());

    match articles_repository.find_by_id(id).await {
        Ok(ok) => match ok {
            Some(_) => match articles_repository.delete(id).await {
                Ok(_) => Ok(HttpResponse::NoContent().body("")),
                Err(err) => Err(AppError::internal_server_error(err.into())),
            },
            None => Err(AppError::not_found()),
        },
        Err(err) => Err(AppError::internal_server_error(err.into())),
    }
}

#[get("/articles/{article_id}/comments")]
async fn comments_index(
    data: web::Data<super::AppState>,
    path_info: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let article_id = path_info.into_inner();
    let dtabase_connection = &data.database_connection;

    let articles_repository = repository::ArticlesRepository::new(dtabase_connection.clone());

    match articles_repository.find_by_id(article_id).await {
        Ok(ok) => match ok {
            Some(_) => {
                let comments_repository =
                    repository::CommentsRepository::new(dtabase_connection.clone());

                match comments_repository.find_all_by_article_id(article_id).await {
                    Ok(comments) => {
                        let response = comments
                            .iter()
                            .map(|comment| CommentIndexResponse {
                                id: comment.id,
                                body: comment.body.clone(),
                            })
                            .collect::<Vec<CommentIndexResponse>>();
                        Ok(HttpResponse::Ok().json(response))
                    }
                    Err(err) => Err(AppError::internal_server_error(err.into())),
                }
            }
            None => Err(AppError::not_found()),
        },
        Err(err) => Err(AppError::internal_server_error(err.into())),
    }
}

#[post("/articles/{article_id}/comments")]
async fn comments_create(
    data: web::Data<super::AppState>,
    path_info: web::Path<i32>,
    comment_form: web::Json<CommentForm>,
) -> Result<HttpResponse, AppError> {
    let article_id = path_info.into_inner();
    let comment_form = comment_form.into_inner();
    let dtabase_connection = &data.database_connection;

    let articles_repository = repository::ArticlesRepository::new(dtabase_connection.clone());

    match articles_repository.find_by_id(article_id).await {
        Ok(ok) => match ok {
            Some(_) => {
                let comments_repository =
                    repository::CommentsRepository::new(dtabase_connection.clone());

                let form = entity::comments::Model {
                    id: 0,
                    article_id,
                    body: comment_form.body,
                };

                match comments_repository.create(form).await {
                    Ok(comment) => {
                        let response = CommentShowResponse {
                            id: comment.id.unwrap(),
                            body: comment.body.unwrap(),
                        };
                        Ok(HttpResponse::Created().json(response))
                    }
                    Err(err) => Err(AppError::internal_server_error(err.into())),
                }
            }
            None => Err(AppError::not_found()),
        },
        Err(err) => Err(AppError::internal_server_error(err.into())),
    }
}

#[get("/articles/{article_id}/comments/{id}")]
async fn comments_show(
    data: web::Data<super::AppState>,
    path_info: web::Path<(i32, i32)>,
) -> Result<HttpResponse, AppError> {
    let path_info = path_info.into_inner();
    let article_id = path_info.0;
    let id = path_info.1;
    let dtabase_connection = &data.database_connection;

    let articles_repository = repository::ArticlesRepository::new(dtabase_connection.clone());

    match articles_repository.find_by_id(article_id).await {
        Ok(ok) => match ok {
            Some(_) => {
                let comments_repository =
                    repository::CommentsRepository::new(dtabase_connection.clone());

                match comments_repository
                    .find_by_article_id_and_id(article_id, id)
                    .await
                {
                    Ok(comment) => match comment {
                        Some(comment) => {
                            let response = CommentShowResponse {
                                id: comment.id,
                                body: comment.body,
                            };
                            Ok(HttpResponse::Ok().json(response))
                        }
                        None => Err(AppError::not_found()),
                    },
                    Err(err) => Err(AppError::internal_server_error(err.into())),
                }
            }
            None => Err(AppError::not_found()),
        },
        Err(err) => Err(AppError::internal_server_error(err.into())),
    }
}

#[patch("/articles/{article_id}/comments/{id}")]
async fn comments_update(
    data: web::Data<super::AppState>,
    path_info: web::Path<(i32, i32)>,
    comment_form: web::Json<CommentForm>,
) -> Result<HttpResponse, AppError> {
    let path_info = path_info.into_inner();
    let article_id = path_info.0;
    let id = path_info.1;
    let dtabase_connection = &data.database_connection;

    let comments_repository = repository::CommentsRepository::new(dtabase_connection.clone());

    match comments_repository
        .find_by_article_id_and_id(article_id, id)
        .await
    {
        Ok(ok) => match ok {
            Some(_) => {
                let comment_form = comment_form.into_inner();

                let form = entity::comments::Model {
                    id,
                    article_id,
                    body: comment_form.body,
                };

                match comments_repository.update(form).await {
                    Ok(_) => Ok(HttpResponse::NoContent().body("")),
                    Err(err) => Err(AppError::internal_server_error(err.into())),
                }
            }
            None => Err(AppError::not_found()),
        },
        Err(err) => Err(AppError::internal_server_error(err.into())),
    }
}

#[delete("/articles/{article_id}/comments/{id}")]
async fn comments_delete(
    data: web::Data<super::AppState>,
    path_info: web::Path<(i32, i32)>,
) -> Result<HttpResponse, AppError> {
    let path_info = path_info.into_inner();
    let article_id = path_info.0;
    let id = path_info.1;
    let dtabase_connection = &data.database_connection;

    let comments_repository = repository::CommentsRepository::new(dtabase_connection.clone());

    match comments_repository
        .find_by_article_id_and_id(article_id, id)
        .await
    {
        Ok(ok) => match ok {
            Some(_) => match comments_repository.delete(article_id, id).await {
                Ok(_) => Ok(HttpResponse::NoContent().body("")),
                Err(err) => Err(AppError::internal_server_error(err.into())),
            },
            None => Err(AppError::not_found()),
        },
        Err(err) => Err(AppError::internal_server_error(err.into())),
    }
}
