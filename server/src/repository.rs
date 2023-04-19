use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter,
    Set,
};

pub struct ArticlesRepository {
    pub database_connection: DatabaseConnection,
}

impl ArticlesRepository {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn find_all(&self) -> Result<Vec<entity::articles::Model>, DbErr> {
        let articles = entity::articles::Entity::find()
            .all(&self.database_connection)
            .await?;

        Ok(articles)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<entity::articles::Model>, DbErr> {
        let article = entity::articles::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await?;

        Ok(article)
    }

    pub async fn create(
        &self,
        form_data: entity::articles::Model,
    ) -> Result<entity::articles::ActiveModel, DbErr> {
        let article = entity::articles::ActiveModel {
            title: Set(form_data.title.to_owned()),
            body: Set(form_data.body.to_owned()),
            ..Default::default()
        }
        .save(&self.database_connection)
        .await?;

        Ok(article)
    }

    pub async fn update(
        &self,
        form_data: entity::articles::Model,
    ) -> Result<entity::articles::ActiveModel, DbErr> {
        let article = entity::articles::Entity::find_by_id(form_data.id)
            .one(&self.database_connection)
            .await?;

        let mut article: entity::articles::ActiveModel = article.unwrap().into();

        article.title = Set(form_data.title.to_owned());
        article.body = Set(form_data.body.to_owned());

        let article: entity::articles::ActiveModel =
            article.update(&self.database_connection).await?.into();

        Ok(article)
    }

    pub async fn delete(&self, id: i32) -> Result<sea_orm::DeleteResult, DbErr> {
        let article = entity::articles::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await?;

        let article: entity::articles::Model = article.unwrap();
        let res: sea_orm::DeleteResult = article.delete(&self.database_connection).await?;

        Ok(res)
    }
}

pub struct CommentsRepository {
    pub database_connection: DatabaseConnection,
}

impl CommentsRepository {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn find_all_by_article_id(
        &self,
        article_id: i32,
    ) -> Result<Vec<entity::comments::Model>, DbErr> {
        let comments = entity::comments::Entity::find()
            .filter(entity::comments::Column::ArticleId.eq(article_id))
            .all(&self.database_connection)
            .await?;

        Ok(comments)
    }
}
