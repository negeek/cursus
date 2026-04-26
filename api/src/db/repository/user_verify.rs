use crate::db::entity::user_verify::{
    self, ActiveModel as UserVerifyOp, Entity as UserVerify, Model as UserVerifyRow,
};
use crate::db::repository::RepositoryTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

pub struct UserVerifyRepository {}

impl RepositoryTrait for UserVerifyRepository {
    type Model = UserVerifyRow;
    type ActiveModel = UserVerifyOp;
    type Entity = UserVerify;

    async fn create(
        &self,
        db: &DatabaseConnection,
        model: Self::ActiveModel,
    ) -> Result<Self::Model, DbErr> {
        model.insert(db).await
    }

    async fn update(
        &self,
        db: &DatabaseConnection,
        model: Self::ActiveModel,
    ) -> Result<Self::Model, DbErr> {
        model.update(db).await
    }

    async fn delete(&self, db: &DatabaseConnection, model: Self::ActiveModel) -> Result<(), DbErr> {
        model.delete(db).await?;
        Ok(())
    }
}

impl UserVerifyRepository {
    pub async fn find_by_code(
        &self,
        db: &DatabaseConnection,
        code: &String,
        user_id: &uuid::Uuid,
    ) -> Result<Option<UserVerifyRow>, DbErr> {
        UserVerify::find()
            .filter(user_verify::COLUMN.code.eq(code))
            .filter(user_verify::COLUMN.user_id.eq(*user_id))
            .one(db)
            .await
    }

    pub async fn find_latest_by_user_id(
        &self,
        db: &DatabaseConnection,
        user_id: &uuid::Uuid,
    ) -> Result<Option<UserVerifyRow>, DbErr> {
        UserVerify::find()
            .filter(user_verify::COLUMN.user_id.eq(*user_id))
            .order_by_id_desc()
            .one(db)
            .await
    }
}
