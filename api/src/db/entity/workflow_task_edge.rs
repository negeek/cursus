use crate::db::entity::{workflow, workflow_task};
use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use uuid::{NoContext, Timestamp, Uuid as uUuid};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workflow_task_edges")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique_key = "unique_workflow_edge")]
    pub workflow_id: Uuid,
    #[sea_orm(belongs_to, from = "workflow_id", to = "id")]
    pub workflow: HasOne<workflow::Entity>,
    #[sea_orm(unique_key = "unique_workflow_edge")]
    pub from_task_id: Uuid,
    #[sea_orm(
        belongs_to,
        relation_enum = "from_task",
        from = "from_task_id",
        to = "id"
    )]
    pub from_task: HasOne<workflow_task::Entity>,
    #[sea_orm(unique_key = "unique_workflow_edge")]
    pub to_task_id: Uuid,
    #[sea_orm(belongs_to, relation_enum = "to_task", from = "to_task_id", to = "id")]
    pub to_task: HasOne<workflow_task::Entity>,
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
