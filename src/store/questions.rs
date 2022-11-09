use crate::types::question::{QuestId, QuestIn, QuestOut, QuestStatus};
use error_handling::ServiceError;
use std::str::FromStr;
use tracing::{event, Level};

use sqlx::postgres::PgRow;
use sqlx::Row;

use super::base::Db;

impl Db {
    pub async fn list(&self, skip: i32, lim: Option<i32>) -> Result<Vec<QuestOut>, ServiceError> {
        let q = sqlx::query("SELECT _id::text, created_at::text, title, content, tags, status::text FROM questions LIMIT $1 OFFSET $2;")
            .bind(lim)
            .bind(skip);
        let q = q.map(|row: PgRow| QuestOut {
            _id: row.get("_id"),
            created_at: row.get("created_at"),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
            status: QuestStatus::from_str(row.get("status")).unwrap(),
            // author: row.get("author")
        });
        let res = q.fetch_all(&self.connection).await;
        if let Err(e) = res {
            event!(Level::ERROR, "List questions query failed: {}", e);
            return Err(ServiceError::DbQueryError);
        }
        Ok(res.unwrap())
    }

    pub async fn add(&self, q: QuestIn) -> Result<QuestId, ServiceError> {
        let quest_status = q.parse_status();
        let q = sqlx::query(
            "INSERT INTO questions (title, content, tags, status) VALUES ($1, $2, $3, $4::question_status) RETURNING _id::text;",
        )
        .bind(q.title)
        .bind(q.content)
        .bind(q.tags)
        .bind(quest_status);
        let q = q.map(|row: PgRow| QuestId::from_str(row.get("_id")).unwrap());

        let res = q.fetch_one(&self.connection).await;
        if let Err(e) = res {
            event!(Level::ERROR, "Add question query failed: {}", e);
            return Err(ServiceError::DbQueryError);
        }
        Ok(res.unwrap())
    }

    pub async fn update(&self, id: QuestId, q: QuestIn) -> Result<(), ServiceError> {
        let quest_status = q.parse_status();
        let q = sqlx::query(
            "UPDATE questions SET title = $1, content = $2, tags = $3, status = $4::question_status WHERE _id = uuid_or_null($5);",
        )
        .bind(q.title)
        .bind(q.content)
        .bind(q.tags)
        .bind(quest_status)
        .bind(id.to_str());
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

    pub async fn delete(&self, id: QuestId) -> Result<(), ServiceError> {
        let q =
            sqlx::query("DELETE FROM questions WHERE _id = uuid_or_null($1);").bind(id.to_str());
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

    pub async fn get(&self, id: QuestId) -> Result<QuestOut, ServiceError> {
        let q =
            sqlx::query("SELECT _id::text, created_at::text, title, content, tags, status::text FROM questions WHERE _id = uuid_or_null($1);")
            .bind(id.to_str());
        let q = q.map(|row: PgRow| QuestOut {
            _id: row.get("_id"),
            created_at: row.get("created_at"),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
            status: QuestStatus::from_str(row.get("status")).unwrap(),
            // author: row.get("author")
        });
        let res = q.fetch_one(&self.connection).await;
        if res.is_err() {
            return Err(ServiceError::ObjectNotFound);
        }
        Ok(res.unwrap())
    }
}
