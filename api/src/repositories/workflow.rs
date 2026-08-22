use jiff::civil::DateTime;
use uuid::Uuid;

use crate::models::{Workflow, now};

pub struct CreateWorkflowParams {
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub run_type: String,
    pub schedule_time: Option<DateTime>,
    pub cron_expression: Option<String>,
    pub cron_next_run: Option<DateTime>,
}

#[derive(Default)]
pub struct UpdateWorkflowParams {
    pub name: Option<String>,
    pub description: Option<String>,
    pub run_type: Option<String>,
    pub schedule_time: Option<DateTime>,
    pub cron_expression: Option<String>,
    pub cron_next_run: Option<DateTime>,
    pub is_active: Option<bool>,
}

pub struct WorkflowRepository;

impl WorkflowRepository {
    pub async fn create(
        &self,
        db: &mut toasty::Db,
        params: CreateWorkflowParams,
    ) -> Result<Workflow, toasty::Error> {
        let timestamp = now();
        toasty::create!(Workflow {
            user_id: params.user_id,
            name: params.name,
            description: params.description,
            run_type: params.run_type,
            schedule_time: params.schedule_time,
            cron_expression: params.cron_expression,
            cron_next_run: params.cron_next_run,
            is_active: true,
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
    ) -> Result<Option<Workflow>, toasty::Error> {
        Workflow::filter(Workflow::fields().id().eq(id))
            .first()
            .exec(db)
            .await
    }

    pub async fn find_by_user_id(
        &self,
        db: &mut toasty::Db,
        user_id: Uuid,
    ) -> Result<Vec<Workflow>, toasty::Error> {
        Workflow::filter(Workflow::fields().user_id().eq(user_id))
            .exec(db)
            .await
    }

    pub async fn update(
        &self,
        db: &mut toasty::Db,
        workflow: &mut Workflow,
        params: UpdateWorkflowParams,
    ) -> Result<(), toasty::Error> {
        let mut stmt = toasty::update!(workflow {
            date_updated: now()
        });
        if let Some(name) = params.name {
            stmt = stmt.name(name);
        }
        if let Some(description) = params.description {
            stmt = stmt.description(description);
        }
        if let Some(run_type) = params.run_type {
            stmt = stmt.run_type(run_type);
        }
        if let Some(schedule_time) = params.schedule_time {
            stmt = stmt.schedule_time(schedule_time);
        }
        if let Some(cron_expression) = params.cron_expression {
            stmt = stmt.cron_expression(cron_expression);
        }
        if let Some(cron_next_run) = params.cron_next_run {
            stmt = stmt.cron_next_run(cron_next_run);
        }
        if let Some(is_active) = params.is_active {
            stmt = stmt.is_active(is_active);
        }
        stmt.exec(db).await
    }

    /// Takes the workflow's steps, edges, and run history with it, since those
    /// relations are declared on the model.
    pub async fn delete(&self, db: &mut toasty::Db, id: Uuid) -> Result<(), toasty::Error> {
        Workflow::filter(Workflow::fields().id().eq(id))
            .delete()
            .exec(db)
            .await
    }
}
