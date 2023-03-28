mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use repository::mongodb_repository::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
}

/*
 * https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5
 * https://codevoweb.com/build-a-crud-api-with-rust-and-mongodb/
 *
 * USANDO CARGO WATCH (no obstante, se requiere refrescar el browser). VER:
 * https://dev.to/jorgecastro/hot-reload-in-rust-with-cargo-watch-5bon
 */
