use crate::document::{Document, DocumentStatus};
use crate::errors::{AppError, ErrorType};

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBAccessManager {
    connection: PooledPg,
}

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager { connection }
    }

    pub fn create_document(&self, dto: Document) -> Result<Document, AppError> {
        use super::schema::documents;

        diesel::insert_into(documents::table)
            .values(&dto)
            .get_result(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while creating document"))
    }

    pub fn list_documents(&self) -> Result<Vec<Document>, AppError> {
        use super::schema::documents::dsl::*;

        documents
            .load(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while listing documents"))
    }

    pub fn update_document_status(
        &self,
        document_id: i64,
        new_status: DocumentStatus,
    ) -> Result<usize, AppError> {
        use super::schema::documents::dsl::*;

        let updated = diesel::update(documents)
            .filter(id.eq(document_id))
            .set(status.eq(new_status))
            .execute(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while updating document status"))?;

        if updated == 0 {
            return Err(AppError::new("Document not found", ErrorType::NotFound));
        }
        return Ok(updated);
    }

    pub fn delete_document(&self, document_id: i64) -> Result<usize, AppError> {
        use super::schema::documents::dsl::*;

        let deleted = diesel::delete(documents.filter(id.eq(document_id)))
            .execute(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while deleting document"))?;

        if deleted == 0 {
            return Err(AppError::new("Document not found", ErrorType::NotFound));
        }
        return Ok(deleted);
    }
}
