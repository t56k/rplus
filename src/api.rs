use crate::data_access::DBAccessManager;
use crate::document::{Document, DocumentStatus};
use crate::AppError;

use serde::Serialize;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct AddDocument {
    pub title: String,
    pub body: String,
    pub status: DocumentStatus,
}

impl AddDocument {
    pub fn to_dto(&self) -> Document {
        Document {
            title: self.title.clone(),
            body: self.body.clone(),
            status: self.status.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateStatus {
    pub status: DocumentStatus,
}

#[derive(Debug, Serialize, Clone)]
pub struct IdResponse {
    pub id: i64,
}

impl IdResponse {
    pub fn new(id: i64) -> IdResponse {
        IdResponse { id }
    }
}

pub async fn add_document(
    db_manager: DBAccessManager,
    new_document: AddDocument,
) -> Result<impl warp::Reply, warp::Rejection> {
    let create_document = new_document.to_dto();

    let id_response = db_manager
        .create_document(create_document)
        .map(|document| IdResponse::new(document.id));

    respond(id_response, warp::http::StatusCode::CREATED)
}

pub async fn update_status(
    document_id: i64,
    db_manager: DBAccessManager,
    status_update: UpdateStatus,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id_response = db_manager
        .update_document_status(document_id, status_update.status)
        .map(|_| IdResponse::new(document_id));

    respond(id_response, warp::http::StatusCode::OK)
}

pub async fn delete_document(
    document_id: i64,
    db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = db_manager.delete_document(document_id).map(|_| {});

    respond(result, warp::http::StatusCode::NO_CONTENT)
}

pub async fn list_documents(
    db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = db_manager.list_documents();

    respond(result, warp::http::StatusCode::OK)
}

fn respond<T: Serialize>(
    result: Result<T, AppError>,
    status: warp::http::StatusCode,
) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => Ok(warp::reply::with_status(
            warp::reply::json(&response),
            status,
        )),
        Err(err) => Err(warp::reject::custom(err)),
    }
}
