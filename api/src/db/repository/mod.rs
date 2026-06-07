pub mod task;
pub mod token;
pub mod user;
pub mod user_verify;
pub mod workflow;
pub mod workflow_state;
pub mod workflow_task;
pub mod workflow_task_edge;
pub mod workflow_task_state;

use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DbErr, PrimaryKeyTrait,
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
        db: &impl ConnectionTrait,
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
        db: &impl ConnectionTrait,
        id: &uuid::Uuid,
    ) -> Result<Option<Self::Model>, DbErr>
    where
        uuid::Uuid: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        Self::Entity::find_by_id(*id).one(db).await
    }

    async fn create(
        &self,
        db: &impl ConnectionTrait,
        model: Self::ActiveModel,
    ) -> Result<Self::Model, DbErr>;
    async fn update(
        &self,
        db: &impl ConnectionTrait,
        model: Self::ActiveModel,
    ) -> Result<Self::Model, DbErr>;
    async fn delete(
        &self,
        db: &impl ConnectionTrait,
        model: Self::ActiveModel,
    ) -> Result<(), DbErr>;
}
