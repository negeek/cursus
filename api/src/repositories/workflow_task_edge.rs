use uuid::Uuid;

use crate::models::{WorkflowTaskEdge, now};

pub struct CreateWorkflowTaskEdgeParams {
    pub workflow_id: Uuid,
    pub from_task_id: Uuid,
    pub to_task_id: Uuid,
}

#[derive(Default)]
pub struct UpdateWorkflowTaskEdgeParams {
    pub from_task_id: Option<Uuid>,
    pub to_task_id: Option<Uuid>,
}

pub struct WorkflowTaskEdgeRepository;

impl WorkflowTaskEdgeRepository {
    pub async fn create(
        &self,
        db: &mut toasty::Db,
        params: CreateWorkflowTaskEdgeParams,
    ) -> Result<WorkflowTaskEdge, toasty::Error> {
        let timestamp = now();
        toasty::create!(WorkflowTaskEdge {
            workflow_id: params.workflow_id,
            from_task_id: params.from_task_id,
            to_task_id: params.to_task_id,
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
    ) -> Result<Option<WorkflowTaskEdge>, toasty::Error> {
        WorkflowTaskEdge::filter(WorkflowTaskEdge::fields().id().eq(id))
            .first()
            .exec(db)
            .await
    }

    /// Every edge in a workflow. Together with the workflow's steps this is the
    /// whole graph, which is what the runner needs before it can order anything.
    pub async fn find_by_workflow_id(
        &self,
        db: &mut toasty::Db,
        workflow_id: Uuid,
    ) -> Result<Vec<WorkflowTaskEdge>, toasty::Error> {
        WorkflowTaskEdge::filter(WorkflowTaskEdge::fields().workflow_id().eq(workflow_id))
            .exec(db)
            .await
    }

    pub async fn update(
        &self,
        db: &mut toasty::Db,
        edge: &mut WorkflowTaskEdge,
        params: UpdateWorkflowTaskEdgeParams,
    ) -> Result<(), toasty::Error> {
        let mut stmt = toasty::update!(edge { date_updated: now() });
        if let Some(from_task_id) = params.from_task_id {
            stmt = stmt.from_task_id(from_task_id);
        }
        if let Some(to_task_id) = params.to_task_id {
            stmt = stmt.to_task_id(to_task_id);
        }
        stmt.exec(db).await
    }

    pub async fn delete(&self, db: &mut toasty::Db, id: Uuid) -> Result<(), toasty::Error> {
        WorkflowTaskEdge::filter(WorkflowTaskEdge::fields().id().eq(id))
            .delete()
            .exec(db)
            .await
    }

    /// Removes every edge touching a step, from either direction.
    ///
    /// This exists because `WorkflowTask` cannot declare `has_many` back to this
    /// table. It would need two of them, incoming and outgoing, and toasty has
    /// no way to tell a pair of `has_many` to the same target apart. So deleting
    /// a step does NOT clean up its edges automatically, and the delete path has
    /// to call this first or it leaves a broken graph behind.
    pub async fn delete_for_task(
        &self,
        db: &mut toasty::Db,
        workflow_task_id: Uuid,
    ) -> Result<(), toasty::Error> {
        WorkflowTaskEdge::filter(
            WorkflowTaskEdge::fields()
                .from_task_id()
                .eq(workflow_task_id),
        )
        .delete()
        .exec(db)
        .await?;

        WorkflowTaskEdge::filter(WorkflowTaskEdge::fields().to_task_id().eq(workflow_task_id))
            .delete()
            .exec(db)
            .await
    }
}
