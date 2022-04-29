use crate::schema::documents;
use crate::user::User;

use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{deserialize, serialize};
use serde_derive::{Deserialize, Serialize};
use std::io::Write;

#[derive(Identifiable, Queryable, Associations, Serialize, PartialEq, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = documents)]
pub struct Document {
    pub id: Option<i64>,
    pub title: String,
    pub body: String,
    pub status: DocumentStatus,
    user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum DocumentStatus {
    Unlocked,
    Locked,
    Expired,
}

impl ToSql<Text, Pg> for DocumentStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W>) -> serialize::Result {
        match *self {
            DocumentStatus::Unlocked => out.write_all(b"UNLOCKED")?,
            DocumentStatus::Locked => out.write_all(b"LOCKED")?,
            DocumentStatus::Expired => out.write_all(b"EXPIRED")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for DocumentStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"UNLOCKED" => Ok(DocumentStatus::Unlocked),
            b"LOCKED" => Ok(DocumentStatus::Locked),
            b"EXPIRED" => Ok(DocumentStatus::Expired),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
