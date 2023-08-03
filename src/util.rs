// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::hash::HashFunction;
use fastcrypto::rsa::Base64UrlUnpadded;
use fastcrypto::rsa::Encoding;
use fastcrypto_zkp::bn254::poseidon::PoseidonWrapper;
use fastcrypto_zkp::bn254::zk_login::big_int_str_to_bytes;
use fastcrypto_zkp::bn254::zk_login::AddressParams;
use fastcrypto_zkp::bn254::zk_login_api::Bn254Fr;
use num_bigint::{BigInt, Sign};
use std::str::FromStr;
use sui_types::crypto::DefaultHash;
use sui_types::crypto::SignatureScheme;

/// Calculate the sui address based on address seed and address params.
pub fn get_user_address(address_seed: String, iss: String, aud: String) -> [u8; 32] {
    let mut hasher = DefaultHash::default();
    hasher.update([SignatureScheme::ZkLoginAuthenticator.flag()]);
    // unwrap is safe here
    hasher.update(bcs::to_bytes(&AddressParams::new(iss, aud)).unwrap());
    hasher.update(big_int_str_to_bytes(&address_seed));
    hasher.finalize().digest
}

/// Return the OIDC URL for the given parameters. Crucially the nonce is computed.
pub fn get_oidc_url(
    eph_pk_bytes: &[u8],
    max_epoch: u64,
    client_id: &str,
    redirect_url: &str,
    jwt_randomness: &str,
) -> String {
    let nonce = get_nonce(eph_pk_bytes, max_epoch, jwt_randomness);
    format!("https://accounts.google.com/o/oauth2/v2/auth?client_id={}&response_type=id_token&redirect_uri={}&scope=open_id&nonce={}", client_id, redirect_url, nonce)
}

/// Return the OIDC URL for the given parameters. Crucially the nonce is computed.
pub fn get_nonce(eph_pk_bytes: &[u8], max_epoch: u64, jwt_randomness: &str) -> String {
    // Nonce is defined as the Base64Url encoded of the poseidon hash of 4 inputs:
    // first half of eph_pubkey bytes in BigInt, second half of eph_pubkey, max_epoch, randomness.

    // todo: split this safely
    let (first_half, second_half) = eph_pk_bytes.split_at(eph_pk_bytes.len() / 2);
    let first_bigint = BigInt::from_bytes_be(Sign::Plus, first_half);
    let second_bigint = BigInt::from_bytes_be(Sign::Plus, second_half);

    let mut poseidon = PoseidonWrapper::new();
    let first = Bn254Fr::from_str(&first_bigint.to_string()).unwrap();
    let second = Bn254Fr::from_str(&second_bigint.to_string()).unwrap();
    let max_epoch = Bn254Fr::from_str(&max_epoch.to_string()).unwrap();
    let jwt_randomness = Bn254Fr::from_str(jwt_randomness).unwrap();

    let hash = poseidon
        .hash(vec![first, second, max_epoch, jwt_randomness])
        .unwrap();
    let data = big_int_str_to_bytes(&hash.to_string());
    let truncated = &data[data.len() - 20..];
    let mut buf = vec![0; Base64UrlUnpadded::encoded_len(truncated)];
    Base64UrlUnpadded::encode(truncated, &mut buf)
        .unwrap()
        .to_string()
}
