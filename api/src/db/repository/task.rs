use crate::db::entity::task::{self, ActiveModel as TaskOp, Entity as Task, Model as TaskRow};
use crate::db::repository::RepositoryTrait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

pub struct TaskRepository {}

impl RepositoryTrait for TaskRepository {
    type Model = TaskRow;
    type ActiveModel = TaskOp;
    type Entity = Task;

    async fn create(
        &self,
        db: &impl ConnectionTrait,
        model: Self::ActiveModel,
    ) -> Result<Self::Model, DbErr> {
        model.insert(db).await
    }

    async fn update(
        &self,
        db: &impl ConnectionTrait,
        model: Self::ActiveModel,
    ) -> Result<Self::Model, DbErr> {
        model.update(db).await
    }

    async fn delete(
        &self,
        db: &impl ConnectionTrait,
        model: Self::ActiveModel,
    ) -> Result<(), DbErr> {
        model.delete(db).await?;
        Ok(())
    }
}

impl TaskRepository {
    pub async fn find_by_user_id(
        &self,
        db: &impl ConnectionTrait,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<TaskRow>, DbErr> {
        Task::find()
            .filter(task::COLUMN.user_id.eq(*user_id))
            .all(db)
            .await
    }
}
