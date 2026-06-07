use crate::db::entity::workflow_state::{
    ActiveModel as WorkflowStateOp, Entity as WorkflowState, Model as WorkflowStateRow,
};
use crate::db::repository::RepositoryTrait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr};

pub struct WorkflowStateRepository {}

impl RepositoryTrait for WorkflowStateRepository {
    type Model = WorkflowStateRow;
    type ActiveModel = WorkflowStateOp;
    type Entity = WorkflowState;

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
