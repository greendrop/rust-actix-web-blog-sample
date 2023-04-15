use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

pub struct ArticlesRepository {
    pub database_connection: DatabaseConnection,
}

impl ArticlesRepository {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    /*
    pub async fn find_all(&self) -> Result<Vec<Articles>, Error> {
        let articles = Articles::Entity::find()
            .all(&self.database_connection)
            .await?;

        Ok(articles)
    }
    */

    pub async fn find_by_id(&self, id: i32) -> Result<Option<entity::articles::Model>, DbErr> {
        let article = entity::articles::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await?;

        Ok(article)
    }

    /*
    pub async fn create(&self, article: &Articles) -> Result<Articles, Error> {
        let article = article
            .save(&self.database_connection)
            .await?;

        Ok(article)
    }

    pub async fn update(&self, article: &Articles) -> Result<Articles, Error> {
        let article = article
            .save(&self.database_connection)
            .await?;

        Ok(article)
    }

    pub async fn delete(&self, article: &Articles) -> Result<Articles, Error> {
        let article = article
            .delete(&self.database_connection)
            .await?;

        Ok(article)
    }
    */
}
