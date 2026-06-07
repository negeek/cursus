use crate::db::entity::workflow_task::{
    ActiveModel as WorkflowTaskOp, Entity as WorkflowTask, Model as WorkflowTaskRow,
};
use crate::db::repository::RepositoryTrait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

pub struct WorkflowTaskRepository {}

impl RepositoryTrait for WorkflowTaskRepository {
    type Model = WorkflowTaskRow;
    type ActiveModel = WorkflowTaskOp;
    type Entity = WorkflowTask;

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

impl WorkflowTaskRepository {
    pub async fn find_by_workflow_id(
        &self,
        db: &impl ConnectionTrait,
        workflow_id: &uuid::Uuid,
    ) -> Result<Vec<WorkflowTaskRow>, DbErr> {
        WorkflowTask::find()
            .filter(WorkflowTask::COLUMN.workflow_id.eq(*workflow_id))
            .all(db)
            .await
    }

    pub async fn find_by_step_name(
        &self,
        db: &impl ConnectionTrait,
        workflow_id: &uuid::Uuid,
        step_name: &String,
    ) -> Result<Option<WorkflowTaskRow>, DbErr> {
        WorkflowTask::find()
            .filter(WorkflowTask::COLUMN.workflow_id.eq(*workflow_id))
            .filter(WorkflowTask::COLUMN.step_name.eq(step_name))
            .one(db)
            .await
    }

    pub async fn find_by_step_id(
        &self,
        db: &impl ConnectionTrait,
        workflow_id: &uuid::Uuid,
        step_id: &uuid::Uuid,
    ) -> Result<Option<WorkflowTaskRow>, DbErr> {
        WorkflowTask::find()
            .filter(WorkflowTask::COLUMN.workflow_id.eq(*workflow_id))
            .filter(WorkflowTask::COLUMN.id.eq(*step_id))
            .one(db)
            .await
    }
}
