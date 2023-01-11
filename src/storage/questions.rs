use crate::types::question::{QuestByUser, QuestOut, QuestStatus};
use crate::types::shared::Id;
use error_handling::ServiceError;
use std::str::FromStr;
use tracing::{event, Level};

use sqlx::postgres::PgRow;
use sqlx::Row;

use super::base::Db;

impl Db {
    pub async fn list_questions(&self, skip: i32, lim: Option<i32>) -> Result<Vec<QuestOut>, ServiceError> {
        let q = sqlx::query(
            "SELECT _id::text, created_at::text, title, content, tags, status::text, author::text FROM questions LIMIT $1 OFFSET $2;",
        )
        .bind(lim)
        .bind(skip);
        let q = q.map(|row: PgRow| QuestOut {
            _id: row.get("_id"),
            created_at: row.get("created_at"),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
            status: QuestStatus::from_str(row.get("status")).unwrap(),
            author: row.get("author"),
        });
        let res = q.fetch_all(&self.connection).await;
        if let Err(e) = res {
            event!(Level::ERROR, "List questions query failed: {}", e);
            return Err(ServiceError::DbQueryError);
        }
        Ok(res.unwrap())
    }

    pub async fn add_question(&self, q: QuestByUser) -> Result<Id, ServiceError> {
        let quest_status = q.parse_status();
        let res = sqlx::query(
            "INSERT INTO questions (title, content, tags, status, author) VALUES ($1, $2, $3, $4::question_status, uuid_or_null($5)) RETURNING _id::text;",
        )
        .bind(q.title)
        .bind(q.content)
        .bind(q.tags)
        .bind(quest_status)
        .bind(q.user_id)
        .map(|row: PgRow| Id::from_str(row.get("_id")).unwrap())
        .fetch_one(&self.connection).await;

        if let Err(e) = res {
            event!(Level::ERROR, "Add question query failed: {}", e);
            return Err(ServiceError::DbQueryError);
        }
        Ok(res.unwrap())
    }

    pub async fn update_question(&self, id: Id, q: QuestByUser, force: bool) -> Result<(), ServiceError> {
        let quest_status = q.parse_status();
        let stmt = match force {
            true => "UPDATE questions SET title = $1, content = $2, tags = $3, status = $4::question_status WHERE _id = uuid_or_null($5);",
            false => "UPDATE questions SET title = $1, content = $2, tags = $3, status = $4::question_status WHERE _id = uuid_or_null($5) AND author = uuid_or_null($6);"
        };
        let q = sqlx::query(stmt)
            .bind(q.title)
            .bind(q.content)
            .bind(q.tags)
            .bind(quest_status)
            .bind(id.to_str())
            .bind(q.user_id);
        let rows_affected = match q.execute(&self.connection).await {
            Err(e) => {
                event!(Level::ERROR, "Update question query failed: {}", e);
                return Err(ServiceError::DbQueryError);
            }
            Ok(res) => res.rows_affected(),
        };
        if rows_affected == 0 {
            return Err(ServiceError::ObjectNotFound);
        }
        Ok(())
    }

    pub async fn delete_question(&self, id: Id, user_id: String, force: bool) -> Result<(), ServiceError> {
        let stmt = match force {
            true => "DELETE FROM questions WHERE _id = uuid_or_null($1);",
            false => "DELETE FROM questions WHERE _id = uuid_or_null($1) and author = uuid_or_null($2);",
        };
        let q = sqlx::query(stmt).bind(id.to_str()).bind(user_id);
        let rows_affected = match q.execute(&self.connection).await {
            Err(e) => {
                event!(Level::ERROR, "Delete question query failed: {}", e);
                return Err(ServiceError::DbQueryError);
            }
            Ok(res) => res.rows_affected(),
        };
        if rows_affected == 0 {
            return Err(ServiceError::ObjectNotFound);
        }
        Ok(())
    }

    pub async fn get_question(&self, id: Id) -> Result<QuestOut, ServiceError> {
        let q = sqlx::query(
            "SELECT _id::text, created_at::text, title, content, tags, status::text, author::text FROM questions WHERE _id = uuid_or_null($1);",
        )
        .bind(id.to_str());
        let q = q.map(|row: PgRow| QuestOut {
            _id: row.get("_id"),
            created_at: row.get("created_at"),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
            status: QuestStatus::from_str(row.get("status")).unwrap(),
            author: row.get("author"),
        });
        let res = q.fetch_one(&self.connection).await;
        if res.is_err() {
            return Err(ServiceError::ObjectNotFound);
        }
        Ok(res.unwrap())
    }
}
