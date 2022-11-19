use error_handling::handle_err;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http, Filter};

mod auth_providers;
mod routes;
mod store;
mod text_processing;
mod types;

#[tokio::main]
async fn main() {
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "customer_care=warn,warp=error".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_methods(vec![http::Method::PUT, http::Method::DELETE])
        .allow_origins(vec!["http://front-end-service:3000"])
        .allow_header("content-type");

    let token_issuer = auth_providers::jwt::JWTAuth::new().expect("JWT auth key issue");
    let token_checker = token_issuer.clone();

    let db = store::base::Db::from_env().await;
    db.run_migrations().await;
    let db_filter = warp::any().map(move || db.clone());

    let moderator_key = std::env::var("MODERATOR_AUTH_KEY").expect("MODERATOR_AUTH_KEY");

    let add_usr = warp::path!("users")
        .and(warp::post())
        .and(warp::body::json())
        .and(routes::auth::parse_auth_headers())
        .and(db_filter.clone())
        .and(warp::any().map(move || moderator_key.clone()))
        .and_then(routes::users::add_user);

    let login_usr = warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(db_filter.clone())
        .and(warp::any().map(move || token_issuer.clone()))
        .and_then(routes::auth::login);

    let list_quest = warp::path!("questions")
        .and(warp::get())
        .and(warp::query())
        .and(db_filter.clone())
        .and_then(routes::questions::list_guestions);

    let add_quest = warp::path!("questions")
        .and(warp::post())
        .and(routes::auth::authenticate(token_checker.clone()))
        .and(db_filter.clone())
        .and(warp::body::json())
        .and_then(routes::questions::add_question);

    let upd_quest = warp::put()
        .and(warp::path("questions"))
        .and(routes::auth::authenticate(token_checker.clone()))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(db_filter.clone())
        .and(warp::body::json())
        .and_then(routes::questions::update_question);

    let del_quest = warp::delete()
        .and(warp::path("questions"))
        .and(routes::auth::authenticate(token_checker))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(db_filter.clone())
        .and_then(routes::questions::delete_question);

    let get_quest = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(db_filter.clone())
        .and_then(routes::questions::get_question);

    let routes = add_usr
        .or(login_usr)
        .or(list_quest)
        .or(add_quest)
        .or(upd_quest)
        .or(del_quest)
        .or(get_quest)
        .with(cors)
        .recover(handle_err)
        .with(warp::trace::request());

    warp::serve(routes).run(([0, 0, 0, 0], 7878)).await;
}
