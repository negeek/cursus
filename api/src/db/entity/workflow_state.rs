use crate::db::entity::workflow;
use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use uuid::{NoContext, Timestamp, Uuid as uUuid};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workflow_states")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub workflow_id: Uuid,
    #[sea_orm(belongs_to, from = "workflow_id", to = "id")]
    pub workflow: HasOne<workflow::Entity>,
    pub status: String, // "pending", "running", "success", "failed"
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
