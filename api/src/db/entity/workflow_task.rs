use crate::db::entity::{task, workflow};
use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use uuid::{NoContext, Timestamp, Uuid as uUuid};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workflow_tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique_key = "unique_workflow_task")]
    pub workflow_id: Uuid,
    #[sea_orm(belongs_to, from = "workflow_id", to = "id")]
    pub workflow: HasOne<workflow::Entity>,
    #[sea_orm(unique_key = "unique_workflow_task")]
    pub task_id: Uuid,
    #[sea_orm(belongs_to, from = "task_id", to = "id")]
    pub task: HasOne<task::Entity>,
    pub task_body: Option<Json>, // override task body for this workflow
    pub run_position: i32,       // position of the task in the workflow run order
    #[sea_orm(nullable)]
    pub retry_count: Option<i32>,
    #[sea_orm(nullable)]
    pub retry_delay_secs: Option<i32>,
    pub date_created: DateTimeUtc,
    pub date_updated: DateTimeUtc,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Will be triggered before insert / update
    async fn before_save<C>(mut self, _: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.id = Set(uUuid::new_v7(Timestamp::now(NoContext)));
            self.date_created = Set(chrono::Utc::now());
            self.date_updated = Set(chrono::Utc::now());
        }
        self.date_updated = Set(chrono::Utc::now());
        Ok(self)
    }
}
