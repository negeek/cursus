use uuid::Uuid;

use crate::dtos::error::workflow_task_edge::WorkflowTaskEdgeServiceError as Error;
use crate::dtos::workflow_task_edge::WorkflowTaskEdgeRequest;
use crate::models::{WorkflowTask, WorkflowTaskEdge};
use crate::repositories::workflow::WorkflowRepository;
use crate::repositories::workflow_task::WorkflowTaskRepository;
use crate::repositories::workflow_task_edge::{
    CreateWorkflowTaskEdgeParams, UpdateWorkflowTaskEdgeParams, WorkflowTaskEdgeRepository,
};
use crate::util::type_conv::parse_uuid;

pub struct WorkflowTaskEdgeService {
    workflows: WorkflowRepository,
    workflow_tasks: WorkflowTaskRepository,
    edges: WorkflowTaskEdgeRepository,
}

impl Default for WorkflowTaskEdgeService {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowTaskEdgeService {
    pub fn new() -> Self {
        Self {
            workflows: WorkflowRepository,
            workflow_tasks: WorkflowTaskRepository,
            edges: WorkflowTaskEdgeRepository,
        }
    }

    /// Confirms the caller owns the workflow they are editing the graph of.
    async fn assert_owns_workflow(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: Uuid,
    ) -> Result<(), Error> {
        let workflow = self
            .workflows
            .find_by_id(db, workflow_id)
            .await?
            .ok_or(Error::WorkflowNotFound)?;

        if workflow.user_id.to_string() != user_id {
            return Err(Error::NotWorkflowOwner);
        }
        Ok(())
    }

    /// Resolves both endpoints of a proposed edge and checks it is a legal one.
    ///
    /// Every rule here protects the graph rather than the request: both ends
    /// must be steps of this workflow, an edge cannot point at itself, and the
    /// same pair cannot be joined twice.
    async fn resolve_endpoints(
        &self,
        db: &mut toasty::Db,
        workflow_id: Uuid,
        request: &WorkflowTaskEdgeRequest,
    ) -> Result<(WorkflowTask, WorkflowTask), Error> {
        let from_task_id = parse_uuid(&request.from_task_id)
            .map_err(|_| Error::InvalidId(request.from_task_id.clone()))?;
        let to_task_id = parse_uuid(&request.to_task_id)
            .map_err(|_| Error::InvalidId(request.to_task_id.clone()))?;

        if from_task_id == to_task_id {
            return Err(Error::SelfLoop);
        }

        // Scoped to the workflow, so a step belonging to a different workflow is
        // simply not found here rather than silently linking two graphs.
        let from_task = self
            .workflow_tasks
            .find_in_workflow(db, workflow_id, from_task_id)
            .await?
            .ok_or(Error::StepOutsideWorkflow)?;

        let to_task = self
            .workflow_tasks
            .find_in_workflow(db, workflow_id, to_task_id)
            .await?
            .ok_or(Error::StepOutsideWorkflow)?;

        Ok((from_task, to_task))
    }

    pub async fn create_workflow_task_edge(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: &str,
        request: WorkflowTaskEdgeRequest,
    ) -> Result<WorkflowTaskEdge, Error> {
        let workflow_id =
            parse_uuid(workflow_id).map_err(|_| Error::InvalidId(workflow_id.to_string()))?;
        self.assert_owns_workflow(db, user_id, workflow_id).await?;

        let (from_task, to_task) = self.resolve_endpoints(db, workflow_id, &request).await?;

        if self
            .edges
            .find_between(db, workflow_id, from_task.id, to_task.id)
            .await?
            .is_some()
        {
            return Err(Error::DuplicateEdge);
        }

        let edge = self
            .edges
            .create(
                db,
                CreateWorkflowTaskEdgeParams {
                    workflow_id,
                    from_task_id: from_task.id,
                    to_task_id: to_task.id,
                },
            )
            .await?;
        Ok(edge)
    }

    pub async fn edit_workflow_task_edge(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: &str,
        edge_id: &str,
        request: WorkflowTaskEdgeRequest,
    ) -> Result<WorkflowTaskEdge, Error> {
        let workflow_id =
            parse_uuid(workflow_id).map_err(|_| Error::InvalidId(workflow_id.to_string()))?;
        let edge_id = parse_uuid(edge_id).map_err(|_| Error::InvalidId(edge_id.to_string()))?;
        self.assert_owns_workflow(db, user_id, workflow_id).await?;

        let mut edge = self
            .edges
            .find_by_id(db, edge_id)
            .await?
            .filter(|e| e.workflow_id == workflow_id)
            .ok_or(Error::EdgeNotFound)?;

        let (from_task, to_task) = self.resolve_endpoints(db, workflow_id, &request).await?;

        // An edge already joining this pair is only a conflict if it is a
        // different edge. Rewriting an edge to the endpoints it already has is
        // a no-op, not a duplicate.
        if let Some(existing) = self
            .edges
            .find_between(db, workflow_id, from_task.id, to_task.id)
            .await?
            && existing.id != edge.id
        {
            return Err(Error::DuplicateEdge);
        }

        self.edges
            .update(
                db,
                &mut edge,
                UpdateWorkflowTaskEdgeParams {
                    from_task_id: Some(from_task.id),
                    to_task_id: Some(to_task.id),
                },
            )
            .await?;

        Ok(edge)
    }

    pub async fn delete_workflow_task_edge(
        &self,
        db: &mut toasty::Db,
        user_id: &str,
        workflow_id: &str,
        edge_id: &str,
    ) -> Result<(), Error> {
        let workflow_id =
            parse_uuid(workflow_id).map_err(|_| Error::InvalidId(workflow_id.to_string()))?;
        let edge_id = parse_uuid(edge_id).map_err(|_| Error::InvalidId(edge_id.to_string()))?;
        self.assert_owns_workflow(db, user_id, workflow_id).await?;

        self.edges
            .find_by_id(db, edge_id)
            .await?
            .filter(|e| e.workflow_id == workflow_id)
            .ok_or(Error::EdgeNotFound)?;

        self.edges.delete(db, edge_id).await?;
        Ok(())
    }
}
