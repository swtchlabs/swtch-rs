// src/identity/models.rs
use ethers::types::Address;

#[derive(Debug, Clone)]
pub struct Identity {
    pub address: Address,
    pub owner: Address,
    pub claims_contract: Address,
    pub did_document: String,
}