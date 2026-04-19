pub mod user;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, PrimaryKeyTrait,
    entity::{EntityTrait, ModelTrait},
};

#[allow(async_fn_in_trait)]
pub trait RepositoryTrait {
    type Model: ModelTrait;
    type ActiveModel: ActiveModelTrait;
    type Entity: EntityTrait<Model = Self::Model>;

    /// Find a record by its primary key (i32)
    async fn find_by_i32(
        &self,
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<Self::Model>, DbErr>
    where
        i32: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        Self::Entity::find_by_id(id).one(db).await
    }

    /// Find a record by its primary key (Uuid)
    async fn find_by_uuid(
        &self,
        db: &DatabaseConnection,
        id: uuid::Uuid,
    ) -> Result<Option<Self::Model>, DbErr>
    where
        uuid::Uuid: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        Self::Entity::find_by_id(id).one(db).await
    }

    async fn create(
        &self,
        db: &DatabaseConnection,
        model: Self::ActiveModel,
    ) -> Result<Self::Model, DbErr>;
    async fn update(
        &self,
        db: &DatabaseConnection,
        model: Self::ActiveModel,
    ) -> Result<Self::Model, DbErr>;
    async fn delete(&self, db: &DatabaseConnection, model: Self::ActiveModel) -> Result<(), DbErr>;
}
