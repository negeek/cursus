use crate::db::entity::user::{ActiveModel as UserOp, Entity as User, Model as UserRow};
use crate::db::repository::RepositoryTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};

pub struct UserRepository {}

impl RepositoryTrait for UserRepository {
    type Model = UserRow;
    type ActiveModel = UserOp;
    type Entity = User;

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

impl UserRepository {
    pub async fn find_by_email(
        &self,
        db: &DatabaseConnection,
        email: &String,
    ) -> Result<Option<UserRow>, DbErr> {
        User::find_by_email(email).one(db).await
    }
}
