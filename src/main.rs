use error_handling::handle_err;
use warp::{http, Filter};

mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let log = warp::log::custom(|info| {
        log::error!(
            "{} {} {} -- {:?} -- from {:?} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers(),
        );
    });

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
        .recover(handle_err)
        .with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}
