#![no_std]
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloc::string::String;
use stylus_sdk::prelude::*;
use alloy_primitives::{Address, U256};

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn main() {
    // Initialize the logger
    stylus_sdk::init();
    
    // Your contract deployment logic here
    let result = stylus_sdk::evm::log("ERC20 Contract Deployed");
    if let Err(e) = result {
        stylus_sdk::evm::log(&format!("Error: {:?}", e));
    }
} 