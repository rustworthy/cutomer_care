use error_handling::handle_err;
use warp::{http, Filter};

mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() {
    env_logger::init();

    log::error!("This is error");
    log::info!("THis is info!");
    log::warn!("This is warning");

    let store = store::Store::new_arc();
    let store_filter = warp::any().map(move || store::Store::clone(&store));
    let cors = warp::cors()
        .allow_methods(vec![http::Method::PUT, http::Method::DELETE])
        .allow_origins(vec!["http://front-end-service:3000"])
        .allow_header("content-type");

    let list_quest = warp::path!("questions")
        .and(warp::get())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::questions::list_guestions);

    let add_quest = warp::path!("questions")
        .and(warp::post())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::questions::add_question);

    let upd_quest = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::questions::update_question);

    let del_quest = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::questions::delete_question);

    let get_quest = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::questions::get_question);

    let routes = list_quest
        .or(add_quest)
        .or(upd_quest)
        .or(del_quest)
        .or(get_quest)
        .with(cors)
        .recover(handle_err);

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}

// mod filters {
//     use warp::Filter;
//     use super::handlers;

//     pub fn list_questions() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//         warp::get()
//         .and(warp::path("questions"))
//         .and(warp::path::end())
//         .and(warp::query())
//         .and(store_filter.clone())
//         .and_then(handlers::list_guestions);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::filters;
//     use warp::test::request;
//     use warp::http::StatusCode;

//     #[tokio::test]
//     async fn test_list_questions() {
//         let api = filters::list_questions();
//         let resp = request().method("GET").path("/questions").reply(&api).await;
//         assert_eq!(resp.status(), StatusCode::ACCEPTED)
//     }
// }
