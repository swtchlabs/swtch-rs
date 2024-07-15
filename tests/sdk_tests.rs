// tests/sdk_tests.rs

use swtch_sdk::SwtchSDK;
use ethers::types::{H160, U256};

#[test]
fn test_add_configuration() {
    let mut sdk = SwtchSDK::new();
    let result = sdk.add_configuration(
        "ethereum_sepolia",
        "ethereum",
        "sepolia",
        "https://sepolia.infura.io/v3/YOUR-PROJECT-ID",
        "your_public_key",
        "your_private_key"
    );
    assert!(result.is_ok());
}

#[test]
fn test_add_invalid_chain_configuration() {
    let mut sdk = SwtchSDK::new();
    let result = sdk.add_configuration(
        "invalid_chain",
        "invalid",
        "mainnet",
        "https://invalid.infura.io/v3/YOUR-PROJECT-ID",
        "your_public_key",
        "your_private_key"
    );
    assert!(result.is_err());
}

#[test]
fn test_use_configuration() {
    let mut sdk = SwtchSDK::new();
    sdk.add_configuration(
        "ethereum_mainnet",
        "ethereum",
        "mainnet",
        "https://mainnet.infura.io/v3/YOUR-PROJECT-ID",
        "your_public_key",
        "your_private_key"
    ).unwrap();
    
    let result = sdk.use_configuration("ethereum_mainnet");
    assert!(result.is_ok());
}

#[test]
fn test_use_nonexistent_configuration() {
    let mut sdk = SwtchSDK::new();
    let result = sdk.use_configuration("nonexistent_config");
    assert!(result.is_err());
}