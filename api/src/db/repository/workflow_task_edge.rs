use crate::db::entity::workflow_task_edge::{
    ActiveModel as WorkflowTaskEdgeOp, Column, Entity as WorkflowTaskEdge,
    Model as WorkflowTaskEdgeRow,
};
use crate::db::repository::RepositoryTrait;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

pub struct WorkflowTaskEdgeRepository {}

impl RepositoryTrait for WorkflowTaskEdgeRepository {
    type Model = WorkflowTaskEdgeRow;
    type ActiveModel = WorkflowTaskEdgeOp;
    type Entity = WorkflowTaskEdge;

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

impl WorkflowTaskEdgeRepository {
    pub async fn find_by_from_and_to(
        &self,
        db: &impl ConnectionTrait,
        workflow_id: &uuid::Uuid,
        from_task_id: &uuid::Uuid,
        to_task_id: &uuid::Uuid,
    ) -> Result<Option<WorkflowTaskEdgeRow>, DbErr> {
        WorkflowTaskEdge::find()
            .filter(Column::WorkflowId.eq(*workflow_id))
            .filter(Column::FromTaskId.eq(*from_task_id))
            .filter(Column::ToTaskId.eq(*to_task_id))
            .one(db)
            .await
    }

    pub async fn find_by_workflow_id(
        &self,
        db: &impl ConnectionTrait,
        workflow_id: &uuid::Uuid,
    ) -> Result<Vec<WorkflowTaskEdgeRow>, DbErr> {
        WorkflowTaskEdge::find()
            .filter(Column::WorkflowId.eq(*workflow_id))
            .all(db)
            .await
    }
}
