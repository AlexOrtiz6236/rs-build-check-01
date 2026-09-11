#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]

extern crate alloc;

use stylus_sdk::{host::VM, prelude::*};

#[entrypoint]
fn user_main(input: Vec<u8>, _vm: VM) -> Result<Vec<u8>, Vec<u8>> {
    Ok(input)
}

#[cfg(feature = "export-abi")]
fn main() {
    stylus_sdk::export_abi();
}
