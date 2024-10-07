use diesel::prelude::*;
use rocket::{
    get, post,
    response::status::{Created, Forbidden, NotFound},
    serde::json::Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    establish_connection,
    models::user::{NewUser, User},
};

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub enum UserResponse {
    NotFound(String),
    AlreadyTaken(String),
}

#[get("/?<id>")]
pub fn get_user(id: String) -> Result<Json<User>, NotFound<Json<UserResponse>>> {
    use crate::schema::users;

    match users::table
        .find(&id)
        .first::<User>(&mut establish_connection())
    {
        Ok(user) => Ok(Json(user)),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                return Err(NotFound(Json(UserResponse::NotFound(id))));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

#[post("/new", format = "json", data = "<user>")]
pub fn new_user(user: Json<NewUser>) -> Result<Created<Json<User>>, Forbidden<Json<UserResponse>>> {
    use crate::schema::users;

    let user = user.into_inner();

    let mut conn = establish_connection();

    if users::table.find(&user.id).first::<User>(&mut conn).is_ok() {
        return Err(Forbidden(Json(UserResponse::AlreadyTaken(user.id))));
    }

    match diesel::insert_into(users::table)
        .values(&user)
        .get_result::<User>(&mut conn)
    {
        Ok(user) => Ok(Created::new("").body(Json(user))),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
