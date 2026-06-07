use crate::db::entity::workflow::{
    ActiveModel as WorkflowOp, Entity as Workflow, Model as WorkflowRow,
};
use crate::db::entity::workflow_task::{Entity as WorkflowTask, Model as WorkflowTaskRow};
use crate::db::entity::workflow_task_edge::{
    Entity as WorkflowTaskEdge, Model as WorkflowTaskEdgeRow,
};
use crate::db::repository::RepositoryTrait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

pub struct WorkflowRepository {}

impl RepositoryTrait for WorkflowRepository {
    type Model = WorkflowRow;
    type ActiveModel = WorkflowOp;
    type Entity = Workflow;

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

impl WorkflowRepository {
    pub async fn find_by_user_id(
        &self,
        db: &impl ConnectionTrait,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<WorkflowRow>, DbErr> {
        Workflow::find()
            .filter(Workflow::COLUMN.user_id.eq(*user_id))
            .all(db)
            .await
    }

    pub async fn load_tasks_and_edges(
        &self,
        db: &impl ConnectionTrait,
        workflow: WorkflowRow,
    ) -> Result<(WorkflowRow, Vec<WorkflowTaskRow>, Vec<WorkflowTaskEdgeRow>), DbErr> {
        let workflow_id = workflow.id;
        let tasks = WorkflowTask::find()
            .filter(WorkflowTask::COLUMN.workflow_id.eq(workflow_id))
            .all(db)
            .await?;
        let edges = WorkflowTaskEdge::find()
            .filter(WorkflowTaskEdge::COLUMN.workflow_id.eq(workflow_id))
            .all(db)
            .await?;
        Ok((workflow, tasks, edges))
    }
}
