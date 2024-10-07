use diesel::prelude::*;
use rocket::serde::{self, Serialize};
use serde::Deserialize;

use crate::schema::users;

mod proxy {
    use std::io::Write;

    use diesel::deserialize::{self, FromSql};
    use diesel::pg::{Pg, PgValue};
    use diesel::serialize;
    use diesel::serialize::{IsNull, Output, ToSql};
    use diesel::sql_types::Jsonb;
    use diesel::{AsExpression, FromSqlRow};

    use super::serde::json::serde_json;
    use super::UserMetadata;

    #[derive(FromSqlRow, AsExpression)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Jsonb)]
    #[allow(dead_code)]
    struct UserMetadataProxy(UserMetadata);

    impl FromSql<Jsonb, Pg> for UserMetadata {
        fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
            let bytes = bytes.as_bytes();

            if bytes[0] != 1 {
                return Err("Unsupported JSONB encoding version".into());
            }

            serde_json::from_slice(&bytes[1..]).map_err(Into::into)
        }
    }

    impl ToSql<Jsonb, Pg> for UserMetadata {
        fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
            out.write_all(&[1])?;
            serde_json::to_writer(out, self)
                .map(|_| IsNull::No)
                .map_err(Into::into)
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]

pub struct UserMetadata {
    pub avatar_url: Option<String>,
}

#[derive(Queryable, Deserialize, Serialize, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub metadata: UserMetadata,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub metadata: UserMetadata,
}
