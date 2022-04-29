#[macro_use]
extern crate diesel;

mod api;
mod brand;
mod data_access;
mod document;
mod errors;
mod schema;
mod user;

use serde::de::DeserializeOwned;
use std::env;
use warp::{reject, Filter};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::api::{AddDocument, UpdateStatus};
use crate::data_access::DBAccessManager;
use crate::errors::{AppError, ErrorType};

type PgPool = Pool<ConnectionManager<PgConnection>>;

fn pg_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

fn with_db_access_manager(
    pool: PgPool,
) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {
            match pool.get() {
                Ok(conn) => Ok(DBAccessManager::new(conn)),
                Err(err) => Err(reject::custom(AppError::new(
                    format!("Error getting connection from pool: {}", err.to_string()).as_str(),
                    ErrorType::Internal,
                ))),
            }
        })
}

fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env not set");
    let pg_pool = pg_pool(database_url.as_str());
    let routes = api_filters(pg_pool).recover(errors::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn api_filters(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / ..).and(
        add_document(pool.clone())
            .or(update_status(pool.clone()))
            .or(delete_document(pool.clone()))
            .or(list_documents(pool)),
    )
}

/// POST /documents
fn add_document(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("documents")
        .and(warp::post())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<AddDocument>())
        .and_then(api::add_document)
}

/// GET /documents
fn list_documents(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("documents")
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(api::list_documents)
}

/// PUT /documents/:id
fn update_status(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("documents" / i64)
        .and(warp::put())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<UpdateStatus>())
        .and_then(api::update_status)
}

/// DELETE /documents/:id
fn delete_document(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("documents" / i64)
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(api::delete_document)
}
