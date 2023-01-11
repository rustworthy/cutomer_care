use crate::types::{
    auth::Creds,
    shared::Id,
    user::{UserIn, UserOut},
};
use error_handling::ServiceError;
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::str::FromStr;
use tracing::{event, instrument, Level};

async fn get_db_err_code(e: &sqlx::Error) -> u16 {
    if let Some(db_err) = e.as_database_error() {
        return db_err.code().unwrap().parse::<u16>().unwrap();
    }
    0
}

impl super::base::Db {
    #[instrument]
    pub async fn add_user(&self, u: UserIn) -> Result<Id, ServiceError> {
        let res = sqlx::query("INSERT INTO users (email, password, first_name, last_name, is_moderator) VALUES($1, crypt($2, gen_salt('bf', 8)), $3, $4, $5) RETURNING _id::text;")
            .bind(u.email)
            .bind(u.password)
            .bind(u.first_name)
            .bind(u.last_name)
            .bind(u.is_moderator.unwrap_or(false))
            .map(|row: PgRow| Id::from_str(row.get("_id")).unwrap())
            .fetch_one(&self.connection).await;

        if let Err(e) = res {
            //https://www.postgresql.org/docs/current/errcodes-appendix.html
            if get_db_err_code(&e).await == 23505 {
                event!(Level::WARN, "{}", e);
                return Err(ServiceError::ConflictInDb);
            }
            event!(Level::ERROR, "{}", e);
            return Err(ServiceError::DbQueryError);
        }
        Ok(res.unwrap())
    }

    pub async fn get_user_by_creds(&self, creds: Creds) -> Result<UserOut, ServiceError> {
        let res = sqlx::query("SELECT _id::text, created_at::text, email, first_name, last_name, is_moderator, is_staff, is_superuser FROM users WHERE email = $1 AND password = crypt($2, password);")
            .bind(creds.email)
            .bind(creds.password)
            .map(|row: PgRow| {
                UserOut {
                    _id: row.get("_id"),
                    created_at: row.get("created_at"),
                    email: row.get("email"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    is_moderator: row.get("is_moderator"),
                    is_staff: row.get("is_staff"),
                    is_superuser: row.get("is_superuser")
                }
            }).fetch_one(&self.connection).await;
        if let Err(e) = res {
            let no_rows_returned = get_db_err_code(&e).await == 0;
            if no_rows_returned {
                event!(Level::WARN, "{}", e);
                return Err(ServiceError::ObjectNotFound);
            }
            event!(Level::ERROR, "{}", e);
            return Err(ServiceError::DbQueryError);
        }
        Ok(res.unwrap())
    }
}
