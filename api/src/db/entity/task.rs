use crate::db::entity::user;
use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use uuid::{NoContext, Timestamp, Uuid as uUuid};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<user::Entity>,
    pub user_title: String, // user defined title for the task
    #[sea_orm(unique)]
    pub title: String, // system generated unique title for the task, used for identification
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub endpoint: String,
    pub method: String,
    #[sea_orm(nullable)]
    pub body: Option<Json>,
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
