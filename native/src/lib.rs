#![allow(non_snake_case)]

use std::sync::atomic::{AtomicUsize, Ordering};

const ABI_VERSION: i32 = 1;
const CAPABILITIES: u64 = 0x1;

static ACTIVATION_COUNT: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn rs_bootstrap_get_abi_version() -> i32 {
    ABI_VERSION
}

#[no_mangle]
pub extern "C" fn rs_bootstrap_get_capabilities() -> u64 {
    CAPABILITIES
}

#[no_mangle]
pub extern "C" fn rs_bootstrap_activate() -> i32 {
    ACTIVATION_COUNT.fetch_add(1, Ordering::SeqCst);
    0
}
