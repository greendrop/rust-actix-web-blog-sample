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

    pub async fn find_by_article_id_and_id(
        &self,
        article_id: i32,
        id: i32,
    ) -> Result<Option<entity::comments::Model>, DbErr> {
        let comment = entity::comments::Entity::find_by_id(id)
            .filter(entity::comments::Column::ArticleId.eq(article_id))
            .one(&self.database_connection)
            .await?;

        Ok(comment)
    }

    pub async fn create(
        &self,
        form_data: entity::comments::Model,
    ) -> Result<entity::comments::ActiveModel, DbErr> {
        let comment = entity::comments::ActiveModel {
            article_id: Set(form_data.article_id.to_owned()),
            body: Set(form_data.body.to_owned()),
            ..Default::default()
        }
        .save(&self.database_connection)
        .await?;

        Ok(comment)
    }

    pub async fn update(
        &self,
        form_data: entity::comments::Model,
    ) -> Result<entity::comments::ActiveModel, DbErr> {
        let comment = entity::comments::Entity::find_by_id(form_data.id)
            .filter(entity::comments::Column::ArticleId.eq(form_data.article_id))
            .one(&self.database_connection)
            .await?;

        let mut comment: entity::comments::ActiveModel = comment.unwrap().into();

        comment.body = Set(form_data.body.to_owned());

        let comment: entity::comments::ActiveModel =
            comment.update(&self.database_connection).await?.into();

        Ok(comment)
    }

    pub async fn delete(&self, article_id: i32, id: i32) -> Result<sea_orm::DeleteResult, DbErr> {
        let comment = entity::comments::Entity::find_by_id(id)
            .filter(entity::comments::Column::ArticleId.eq(article_id))
            .one(&self.database_connection)
            .await?;

        let comment: entity::comments::Model = comment.unwrap();
        let res: sea_orm::DeleteResult = comment.delete(&self.database_connection).await?;

        Ok(res)
    }
}
