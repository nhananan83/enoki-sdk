use fastcrypto::encoding::{Encoding, Hex};
use fastcrypto::hmac::{hkdf_sha3_256, HkdfIkm};
use fastcrypto::traits::ToFromBytes;
use rocket::State;
use rocket_contrib::json::JsonValue;
use std::sync::atomic::Ordering;

use crate::EnokiConfig;

pub const USER_PIN_LENGTH: usize = 16;
type GetPINRequest = String;

#[derive(Serialize, Deserialize)]
struct GetPINResponse {
    pin: String,
    id: String,
}

#[get("/<username>", format = "json")]
pub fn get_pin(username: GetPINRequest, state: State<EnokiConfig>) -> JsonValue {
    let pin = Hex::encode(
        hkdf_sha3_256(
            &HkdfIkm::from_bytes(&state.seed.clone().unwrap()).unwrap(),
            &[],
            state.app_name.as_bytes(),
            USER_PIN_LENGTH,
        )
        .unwrap(),
    );
    json!(GetPINResponse {
        pin,
        id: username.to_string()
    })
}

#[get("/get_pin_and_id", format = "json")]
pub fn get_pin_and_id(state: State<EnokiConfig>) -> JsonValue {
    let count = state.counter.fetch_add(1, Ordering::Relaxed) + 1;
    let pin = Hex::encode(
        hkdf_sha3_256(
            &HkdfIkm::from_bytes(&[1, 2, 3]).unwrap(),
            &[],
            state.app_name.as_bytes(),
            USER_PIN_LENGTH,
        )
        .unwrap(),
    );
    json!(GetPINResponse {
        pin,
        id: count.to_string()
    })
}
