use crate::db::entity::workflow_task_state::{
    ActiveModel as WorkflowTaskStateOp, Entity as WorkflowTaskState, Model as WorkflowTaskStateRow,
};
use crate::db::repository::RepositoryTrait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr};

pub struct WorkflowTaskStateRepository {}

impl RepositoryTrait for WorkflowTaskStateRepository {
    type Model = WorkflowTaskStateRow;
    type ActiveModel = WorkflowTaskStateOp;
    type Entity = WorkflowTaskState;

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
