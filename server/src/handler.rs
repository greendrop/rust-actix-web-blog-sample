use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
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

#[post("/articles")]
async fn articles_create(
    data: web::Data<super::AppState>,
    article_form: web::Json<ArticleForm>,
) -> impl Responder {
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
            return HttpResponse::Created().json(response);
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

#[patch("/articles/{id}")]
async fn articles_update(
    data: web::Data<super::AppState>,
    id: web::Path<i32>,
    article_form: web::Json<ArticleForm>,
) -> impl Responder {
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
                    Ok(_) => return HttpResponse::NoContent().body(""),
                    Err(_err) => {
                        return HttpResponse::InternalServerError()
                            .json(HttpErrorResponse::internal_server_error())
                    }
                }
            }
            None => return HttpResponse::NotFound().json(HttpErrorResponse::not_found()),
        },
        Err(_err) => {
            return HttpResponse::InternalServerError()
                .json(HttpErrorResponse::internal_server_error())
        }
    }
}

#[delete("/articles/{id}")]
async fn articles_delete(data: web::Data<super::AppState>, id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();
    let dtabase_connection = &data.database_connection;

    let articles_repository = repository::ArticlesRepository::new(dtabase_connection.clone());

    match articles_repository.find_by_id(id).await {
        Ok(ok) => match ok {
            Some(_) => match articles_repository.delete(id).await {
                Ok(_) => return HttpResponse::NoContent().body(""),
                Err(_err) => {
                    return HttpResponse::InternalServerError()
                        .json(HttpErrorResponse::internal_server_error())
                }
            },
            None => return HttpResponse::NotFound().json(HttpErrorResponse::not_found()),
        },
        Err(_err) => {
            return HttpResponse::InternalServerError()
                .json(HttpErrorResponse::internal_server_error())
        }
    }
}

#[get("/articles/{article_id}/comments")]
async fn comments_index(
    data: web::Data<super::AppState>,
    path_info: web::Path<i32>,
) -> impl Responder {
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
                        return HttpResponse::Ok().json(response);
                    }
                    Err(_err) => {
                        return HttpResponse::InternalServerError()
                            .json(HttpErrorResponse::internal_server_error())
                    }
                }
            }
            None => return HttpResponse::NotFound().json(HttpErrorResponse::not_found()),
        },
        Err(_err) => {
            return HttpResponse::InternalServerError()
                .json(HttpErrorResponse::internal_server_error())
        }
    }
}

#[post("/articles/{article_id}/comments")]
async fn comments_create(
    data: web::Data<super::AppState>,
    path_info: web::Path<i32>,
    comment_form: web::Json<CommentForm>,
) -> impl Responder {
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
                        return HttpResponse::Created().json(response);
                    }
                    Err(_err) => {
                        return HttpResponse::InternalServerError()
                            .json(HttpErrorResponse::internal_server_error())
                    }
                }
            }
            None => return HttpResponse::NotFound().json(HttpErrorResponse::not_found()),
        },
        Err(_err) => {
            return HttpResponse::InternalServerError()
                .json(HttpErrorResponse::internal_server_error())
        }
    }
}

#[get("/articles/{article_id}/comments/{id}")]
async fn comments_show(
    data: web::Data<super::AppState>,
    path_info: web::Path<(i32, i32)>,
) -> impl Responder {
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
                            return HttpResponse::Ok().json(response);
                        }
                        None => {
                            return HttpResponse::NotFound().json(HttpErrorResponse::not_found())
                        }
                    },
                    Err(_err) => {
                        return HttpResponse::InternalServerError()
                            .json(HttpErrorResponse::internal_server_error())
                    }
                }
            }
            None => return HttpResponse::NotFound().json(HttpErrorResponse::not_found()),
        },
        Err(_err) => {
            return HttpResponse::InternalServerError()
                .json(HttpErrorResponse::internal_server_error())
        }
    }
}

#[patch("/articles/{article_id}/comments/{id}")]
async fn comments_update(
    data: web::Data<super::AppState>,
    path_info: web::Path<(i32, i32)>,
    comment_form: web::Json<CommentForm>,
) -> impl Responder {
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
                    Ok(_) => return HttpResponse::NoContent().body(""),
                    Err(_err) => {
                        return HttpResponse::InternalServerError()
                            .json(HttpErrorResponse::internal_server_error())
                    }
                }
            }
            None => return HttpResponse::NotFound().json(HttpErrorResponse::not_found()),
        },
        Err(_err) => {
            return HttpResponse::InternalServerError()
                .json(HttpErrorResponse::internal_server_error())
        }
    }
}

#[delete("/articles/{article_id}/comments/{id}")]
async fn comments_delete(
    data: web::Data<super::AppState>,
    path_info: web::Path<(i32, i32)>,
) -> impl Responder {
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
                Ok(_) => return HttpResponse::NoContent().body(""),
                Err(_err) => {
                    return HttpResponse::InternalServerError()
                        .json(HttpErrorResponse::internal_server_error());
                }
            },
            None => return HttpResponse::NotFound().json(HttpErrorResponse::not_found()),
        },
        Err(_err) => {
            return HttpResponse::InternalServerError()
                .json(HttpErrorResponse::internal_server_error());
        }
    }
}
