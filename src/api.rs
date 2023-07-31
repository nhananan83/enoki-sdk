use crate::EnokiConfig;
use fastcrypto::encoding::Hex;
use fastcrypto::hmac::{hkdf_sha3_256, HkdfIkm};
use fastcrypto::traits::ToFromBytes;
use rocket::State;
use rocket_contrib::json::JsonValue;
use serde_with::serde_as;
use std::sync::atomic::Ordering;

pub const USER_PIN_LENGTH: usize = 16;

type GetPINRequest = String;

#[serde_as]
#[derive(Serialize, Deserialize)]
struct GetPINResponse {
    #[serde_as(as = "Hex")]
    pin: Vec<u8>,
    #[serde_as(as = "Hex")]
    id: Vec<u8>,
}

pub fn derive_user_pin(master_seed: &[u8], id: &[u8], app_name: &[u8]) -> Vec<u8> {
    hkdf_sha3_256(
        &HkdfIkm::from_bytes(master_seed).unwrap(),
        app_name,
        id,
        USER_PIN_LENGTH,
    )
    .unwrap()
}

#[get("/get_pin/<username>", format = "json")]
pub fn get_pin(username: GetPINRequest, state: State<EnokiConfig>) -> JsonValue {
    json!(GetPINResponse {
        pin: derive_user_pin(&state.seed, username.as_bytes(), state.app_name.as_bytes()),
        id: username.as_bytes().to_vec()
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
