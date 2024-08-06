// src/reputation/models.rs

use ethers::types::{Address, U256};

#[derive(Debug, Clone)]
pub struct Reputation {
    pub did: Address,
    pub consumer_score: U256,
    pub producer_score: U256,
    pub eth_balance: U256,
}

impl Reputation {
    pub fn new(did: Address, consumer_score: U256, producer_score: U256, eth_balance: U256) -> Self {
        Self {
            did,
            consumer_score,
            producer_score,
            eth_balance,
        }
    }
}