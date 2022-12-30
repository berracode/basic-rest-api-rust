use rocket::{self, routes};

use crate::{connection, schema::people};

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/people",
               routes![crate::people::handler::all,
               crate::people::handler::get,
               crate::people::handler::post,
               crate::people::handler::put,
               crate::people::handler::delete],
        ).launch();
}