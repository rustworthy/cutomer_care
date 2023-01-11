use std::str::FromStr;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::aux::filter_out_bad_words;
use crate::storage::Db;
use crate::types::pagination::Pagination;
use crate::types::question::QuestIn;
use crate::types::shared::Id;
use crate::types::user::UserTknDetails;

type Params = std::collections::HashMap<String, String>;

pub async fn process_question_text(mut quest_incoming: QuestIn) -> Result<QuestIn, Rejection> {
    let title = tokio::spawn(filter_out_bad_words(quest_incoming.title));
    let content = tokio::spawn(filter_out_bad_words(quest_incoming.content));
    let (title, content) = (title.await.unwrap(), content.await.unwrap());
    quest_incoming.title = title.map_err(warp::reject::custom)?;
    quest_incoming.content = content.map_err(warp::reject::custom)?;
    Ok(quest_incoming)
}

pub async fn list_guestions(query_string_params: Params, db: Db) -> Result<impl Reply, Rejection> {
    let pagination = match query_string_params.is_empty() {
        true => Pagination::default(),
        false => Pagination::parse_from_map(query_string_params)?,
    };
    let questions = db
        .list_questions(pagination.offset, pagination.limit)
        .await
        .map_err(warp::reject::custom)?;

    Ok(warp::reply::json(&questions))
}

pub async fn add_question(user: UserTknDetails, db: Db, mut question: QuestIn) -> Result<impl Reply, Rejection> {
    if !user.is_moderator {
        question = process_question_text(question).await?;
    }
    let question = question.authored_by(user._id);
    let inserted_id = db.add_question(question).await.map_err(warp::reject::custom)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&inserted_id.as_dict()),
        StatusCode::CREATED,
    ))
}

pub async fn update_question(user: UserTknDetails, id: String, db: Db, mut question: QuestIn) -> Result<impl Reply, Rejection> {
    if !user.is_moderator {
        question = process_question_text(question).await?;
    }
    let question = question.authored_by(user._id);
    match db
        .update_question(Id::from_str(&id).unwrap(), question, user.is_moderator)
        .await
    {
        Ok(_) => Ok(warp::reply::with_status("", StatusCode::NO_CONTENT)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_question(user: UserTknDetails, id: String, db: Db) -> Result<impl Reply, Rejection> {
    match db
        .delete_question(Id::from_str(&id).unwrap(), user._id, user.is_moderator)
        .await
    {
        Ok(_) => Ok(warp::reply::with_status("", StatusCode::NO_CONTENT)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_question(id: String, db: Db) -> Result<impl Reply, Rejection> {
    let question = db
        .get_question(Id::from_str(&id).unwrap())
        .await
        .map_err(warp::reject::custom)?;

    Ok(warp::reply::json(&question))
}
