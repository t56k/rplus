use crate::brand::Brand;
use crate::schema::users;

use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{deserialize, serialize};
use serde_derive::{Deserialize, Serialize};
use std::io::Write;

#[derive(Identifiable, Queryable, Associations, Serialize, PartialEq, Debug)]
#[diesel(belongs_to(Brand))]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub client_code: String,
    pub status: UserStatus,
    brand_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum UserStatus {
    Inactive,
    Guest,
    Demo,
    Client,
    Administrator,
}

impl ToSql<Text, Pg> for UserStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W>) -> serialize::Result {
        match *self {
            UserStatus::WantToRead => out.write_all(b"WANT_TO_READ")?,
            UserStatus::Reading => out.write_all(b"READING")?,
            UserStatus::Finished => out.write_all(b"FINISHED")?,
            UserStatus::Rereading => out.write_all(b"REREADING")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for UserStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"WANT_TO_READ" => Ok(UserStatus::WantToRead),
            b"READING" => Ok(UserStatus::Reading),
            b"FINISHED" => Ok(UserStatus::Finished),
            b"REREADING" => Ok(UserStatus::Rereading),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
