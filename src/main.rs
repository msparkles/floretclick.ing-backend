use floretclicking_backend::api::user;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/user", routes![user::get_user, user::new_user])
}
