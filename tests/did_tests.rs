// tests/did_tests.rs

use swtch_sdk::identity::{IdentityManager, Identity};
use ethers::prelude::*;
use std::sync::Arc;
use async_trait::async_trait;

mod common;
use common::{CustomMockProvider, mock_identity_manager, random_address};

#[tokio::test]
async fn test_create_identity_manager() {
    let identity_manager = mock_identity_manager();
    assert!(identity_manager.contract.address() != Address::zero());
}

#[tokio::test]
async fn test_load_identity() {
    let identity_manager = mock_identity_manager();
    let did = random_address();

    let result = identity_manager.load_identity(did).await;
    assert!(result.is_err());
    let error_string = result.unwrap_err().to_string();
    assert!(
        error_string.contains("Mock call") || 
        error_string.contains("provider error"),
        "Unexpected error message: {}",
        error_string
    );
}

#[tokio::test]
async fn test_register_identity() {
    let identity_manager = mock_identity_manager();
    let did = random_address();
    let owner = random_address();
    let document_hash = "test_document_hash".to_string();

    let result = identity_manager.register_identity(did, owner, document_hash).await;
    assert!(result.is_err());
    let error_string = result.unwrap_err().to_string();
    assert!(
        error_string.contains("Mock send_transaction") || 
        error_string.contains("provider error"),
        "Unexpected error message: {}",
        error_string
    );
}

#[tokio::test]
async fn test_sign_and_verify_message() {
    let identity_manager = mock_identity_manager();
    let message = b"Test message";
    
    let signature_result = identity_manager.sign_message(message).await;
    assert!(signature_result.is_ok());
    
    if let Ok(signature) = signature_result {
        let signer = identity_manager.wallet.address();
        assert!(identity_manager.verify_signature(message, &signature, signer));
    }
}