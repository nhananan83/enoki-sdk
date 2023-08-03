// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::EnokiConfig;
use fastcrypto::encoding::Hex;
use fastcrypto::hmac::{hkdf_sha3_256, HkdfIkm};
use fastcrypto::rsa::Base64UrlUnpadded;
use fastcrypto::traits::ToFromBytes;
use rocket::State;
use rocket_contrib::json::JsonValue;
use serde_json::Value;
use serde_with::serde_as;
use std::sync::atomic::Ordering;
pub const USER_PIN_LENGTH: usize = 16;
use fastcrypto::rsa::Encoding as OtherEncoding;

type GetPINRequest = String;

#[serde_as]
#[derive(Serialize, Deserialize)]
struct GetPINResponse {
    #[serde_as(as = "Hex")]
    pin: Vec<u8>,
    #[serde_as(as = "Hex")]
    id: Vec<u8>,
}

#[get("/get_pin/<token>", format = "json")]
pub fn get_pin(token: GetPINRequest, state: State<EnokiConfig>) -> JsonValue {
    // todo: cache jwks and actually verify here
    let parts: Vec<&str> = token.split('.').collect();
    let decoded_header = Base64UrlUnpadded::decode_vec(parts[1]).unwrap();
    let json_header: Value = serde_json::from_slice(&decoded_header).unwrap();
    let mut app_id = json_header["iss"].as_str().unwrap().as_bytes().to_vec();
    app_id.extend_from_slice(json_header["aud"].as_str().unwrap().as_bytes());
    let sub = json_header["sub"].as_str().unwrap().as_bytes();
    json!(GetPINResponse {
        pin: derive_user_pin(&state.seed, &sub, &app_id),
        id: sub.to_vec()
    })
}

#[get("/get_pin_and_id", format = "json")]
pub fn get_pin_and_id(state: State<EnokiConfig>) -> JsonValue {
    let count = state.counter.fetch_add(1, Ordering::Relaxed) + 1;
    json!(GetPINResponse {
        pin: derive_user_pin(&state.seed, &count.to_be_bytes(), state.app_name.as_bytes()),
        id: count.to_be_bytes().to_vec()
    })
}
