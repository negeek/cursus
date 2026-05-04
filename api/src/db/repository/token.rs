use crate::db::entity::token::{
    self, ActiveModel as BlacklistTokenOp, Entity as BlacklistToken, Model as BlacklistTokenRow,
};
use crate::db::repository::RepositoryTrait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, SelectExt};

pub struct BlacklistTokenRepository {}

impl RepositoryTrait for BlacklistTokenRepository {
    type Model = BlacklistTokenRow;
    type ActiveModel = BlacklistTokenOp;
    type Entity = BlacklistToken;

    async fn create(
        &self,
        db: &impl ConnectionTrait,
        model: Self::ActiveModel,
    ) -> Result<Self::Model, DbErr> {
        model.insert(db).await
    }

    async fn update(
        &self,
        db: &impl ConnectionTrait,
        model: Self::ActiveModel,
    ) -> Result<Self::Model, DbErr> {
        model.update(db).await
    }

    async fn delete(
        &self,
        db: &impl ConnectionTrait,
        model: Self::ActiveModel,
    ) -> Result<(), DbErr> {
        model.delete(db).await?;
        Ok(())
    }
}

impl BlacklistTokenRepository {
    pub async fn find_by_jti(
        &self,
        db: &impl ConnectionTrait,
        jti: &str,
    ) -> Result<Option<BlacklistTokenRow>, DbErr> {
        BlacklistToken::find()
            .filter(token::COLUMN.jti.eq(jti))
            .one(db)
            .await
    }

    pub async fn jti_exists(&self, db: &impl ConnectionTrait, jti: &str) -> Result<bool, DbErr> {
        BlacklistToken::find()
            .filter(token::COLUMN.jti.eq(jti))
            .exists(db)
            .await
    }
}
