// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use enoki_sdk::{
    api::{static_rocket_route_info_for_get_pin, static_rocket_route_info_for_get_pin_and_id},
    EnokiConfig,
};
use fastcrypto::encoding::{Base64, Encoding};
use rocket::routes;
use std::{env, sync::atomic::AtomicUsize};

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
