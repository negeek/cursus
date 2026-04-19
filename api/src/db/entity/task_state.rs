use crate::db::entity::{task, workflow_state};
use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use uuid::{NoContext, Timestamp, Uuid as uUuid};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "task_states")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub task_id: Uuid,
    #[sea_orm(belongs_to, from = "task_id", to = "id")]
    pub task: HasOne<task::Entity>,
    pub workflow_state_id: Uuid,
    #[sea_orm(belongs_to, from = "workflow_state_id", to = "id")]
    pub workflow_state: HasOne<workflow_state::Entity>,
    #[sea_orm(nullable)]
    pub result: Option<Json>,
    pub status: String, // "pending", "running", "success", "failed"
    pub retry_attempts: i32,
    #[sea_orm(nullable)]
    pub start_time: Option<DateTimeUtc>,
    #[sea_orm(nullable)]
    pub end_time: Option<DateTimeUtc>,
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
