use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use uuid::{NoContext, Timestamp, Uuid as uUuid};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub email_verified: bool,
    pub password_hash: String,
    pub date_created: DateTimeUtc,
    pub date_updated: DateTimeUtc,
    pub date_joined: DateTimeUtc,
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
            self.date_joined = Set(chrono::Utc::now());
        }
        self.date_updated = Set(chrono::Utc::now());
        Ok(self)
    }
}
