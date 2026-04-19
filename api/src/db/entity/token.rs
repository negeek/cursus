use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blacklisted_tokens")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub jti: String,
    pub token_type: String,
    pub expires_at: DateTimeUtc,
    pub date_created: DateTimeUtc,
    pub date_updated: DateTimeUtc,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.date_created = Set(chrono::Utc::now());
        }
        self.date_updated = Set(chrono::Utc::now());
        Ok(self)
    }
}
