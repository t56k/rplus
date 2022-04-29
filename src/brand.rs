use crate::schema::brands;

use serde_derive::Serialize;

#[derive(Identifiable, Queryable, Serialize, PartialEq, Debug)]
#[diesel(table_name = brands)]
pub struct Brand {
    pub id: Option<i64>,
    pub name: String,
}
