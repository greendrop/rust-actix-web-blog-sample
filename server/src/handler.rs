use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

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

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/articles")]
async fn articles_index(data: web::Data<super::AppState>) -> impl Responder {
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
            return HttpResponse::Ok().json(response);
        }
        Err(_err) => {
            return HttpResponse::InternalServerError()
                .json(HttpErrorResponse::internal_server_error())
        }
    }
}

#[get("/articles/{id}")]
async fn articles_show(data: web::Data<super::AppState>, id: web::Path<i32>) -> impl Responder {
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
                return HttpResponse::Ok().json(response);
            }
            None => return HttpResponse::NotFound().json(HttpErrorResponse::not_found()),
        },
        Err(_err) => {
            return HttpResponse::InternalServerError()
                .json(HttpErrorResponse::internal_server_error())
        }
    }
}
