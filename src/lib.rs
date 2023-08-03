#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::sync::atomic::AtomicUsize;

pub struct EnokiConfig {
    pub seed: Vec<u8>,
    pub counter: AtomicUsize,
}

pub mod api;
pub mod sdk;
#[cfg(test)]
mod tests;
