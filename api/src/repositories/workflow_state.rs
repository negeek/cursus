use jiff::civil::DateTime;
use uuid::Uuid;

use crate::models::{WorkflowState, now};

pub struct CreateWorkflowStateParams {
    pub workflow_id: Uuid,
    pub status: String,
}

#[derive(Default)]
pub struct UpdateWorkflowStateParams {
    pub status: Option<String>,
    pub start_time: Option<DateTime>,
    pub end_time: Option<DateTime>,
}

pub struct WorkflowStateRepository;

impl WorkflowStateRepository {
    pub async fn create(
        &self,
        db: &mut toasty::Db,
        params: CreateWorkflowStateParams,
    ) -> Result<WorkflowState, toasty::Error> {
        let timestamp = now();
        toasty::create!(WorkflowState {
            workflow_id: params.workflow_id,
            status: params.status,
            date_created: timestamp,
            date_updated: timestamp,
        })
        .exec(db)
        .await
    }

    pub async fn find_by_id(
        &self,
        db: &mut toasty::Db,
        id: Uuid,
    ) -> Result<Option<WorkflowState>, toasty::Error> {
        WorkflowState::filter(WorkflowState::fields().id().eq(id))
            .first()
            .exec(db)
            .await
    }

    /// Every run of a workflow, newest first.
    pub async fn find_by_workflow_id(
        &self,
        db: &mut toasty::Db,
        workflow_id: Uuid,
    ) -> Result<Vec<WorkflowState>, toasty::Error> {
        WorkflowState::filter(WorkflowState::fields().workflow_id().eq(workflow_id))
            .order_by(WorkflowState::fields().id().desc())
            .exec(db)
            .await
    }

    pub async fn update(
        &self,
        db: &mut toasty::Db,
        state: &mut WorkflowState,
        params: UpdateWorkflowStateParams,
    ) -> Result<(), toasty::Error> {
        let mut stmt = toasty::update!(state {
            date_updated: now()
        });
        if let Some(status) = params.status {
            stmt = stmt.status(status);
        }
        if let Some(start_time) = params.start_time {
            stmt = stmt.start_time(start_time);
        }
        if let Some(end_time) = params.end_time {
            stmt = stmt.end_time(end_time);
        }
        stmt.exec(db).await
    }

    pub async fn delete(&self, db: &mut toasty::Db, id: Uuid) -> Result<(), toasty::Error> {
        WorkflowState::filter(WorkflowState::fields().id().eq(id))
            .delete()
            .exec(db)
            .await
    }
}
