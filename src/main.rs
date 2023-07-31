#![feature(proc_macro_hygiene, decl_macro)]

use api::{static_rocket_route_info_for_get_pin, static_rocket_route_info_for_get_pin_and_id};
use std::sync::atomic::AtomicUsize;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod api;
#[cfg(test)]
mod tests;

pub struct EnokiConfig {
    app_name: String,
    seed: Option<Vec<u8>>,
    counter: AtomicUsize,
}

// fn main() {
//     rocket::ignite().manage(EnokiConfig { app_name: "sui_wallet".to_string(), seed: None, counter: AtomicUsize::new(0) }).mount("/", routes![get_pin, get_pin_and_id])
//     .launch();
// }

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![get_pin, get_pin_and_id])
        .manage(EnokiConfig {
            app_name: "sui_wallet".to_string(),
            seed: None,
            counter: AtomicUsize::new(0),
        })
}

fn main() {
    rocket().launch();
}
