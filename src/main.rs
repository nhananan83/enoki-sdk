// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![feature(proc_macro_hygiene, decl_macro)]

use api::{static_rocket_route_info_for_get_pin, static_rocket_route_info_for_get_pin_and_id};
use fastcrypto::encoding::{Base64, Encoding};
use std::{env, sync::atomic::AtomicUsize};
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod api;
mod sdk;
#[cfg(test)]
mod tests;

pub struct EnokiConfig {
    seed: Vec<u8>,
    counter: AtomicUsize,
}

fn rocket() -> rocket::Rocket {
    let seed = env::var("SEED").expect("SEED missing");
    rocket::ignite()
        .mount("/", routes![get_pin, get_pin_and_id])
        .manage(EnokiConfig {
            seed: Base64::decode(&seed).expect("Invalid seed"),
            counter: AtomicUsize::new(0),
        })
}

fn main() {
    rocket().launch();
}
