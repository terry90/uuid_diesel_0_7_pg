#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Uuid as UuidDiesel;
use std::io::Write;
use uuid;

#[derive(Debug, AsExpression, PartialEq, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "UuidDiesel"]
pub struct Uuid(uuid::Uuid);

impl ToSql<Uuid, Pg> for Uuid {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_all(self.0.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Uuid, Pg> for Uuid {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        Ok(Uuid(uuid::Uuid::from_slice(bytes)?))
    }
}
