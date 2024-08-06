// src/lib.rs
pub mod crypto;
pub mod context;
pub mod identity;
pub mod transactions;
pub mod utils;

pub use crate::context::{ContextManager, Config, BlockchainConfig, WalletConfig, ChainType, NetworkType, TestnetType};
pub use crate::identity::{IdentityManager, Identity};
pub use crate::transactions::TransactionReceipt as SWTCHTransaction;

use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::types::{Address, Signature, TransactionReceipt, U256};
use std::sync::Arc;

pub struct SwtchSDK {
    context_manager: ContextManager,
    identity_manager: Option<IdentityManager<Provider<Http>>>,
}

impl SwtchSDK {
    pub fn new() -> Self {
        Self {
            context_manager: ContextManager::new(),
            identity_manager: None,
        }
    }

    pub fn add_configuration(
        &mut self,
        name: &str,
        chain: &str,
        network: &str,
        provider_url: &str,
        public_key: &str,
        private_key: &str
    ) -> Result<(), String> {
        let chain_type = match chain.to_lowercase().as_str() {
            "ethereum" => ChainType::Ethereum,
            "polygon" => ChainType::Polygon,
            "avalanche" => ChainType::Avalanche,
            _ => return Err(format!("Unsupported chain type: {}", chain)),
        };

        let network_type = match network.to_lowercase().as_str() {
            "mainnet" => NetworkType::Mainnet,
            "sepolia" => NetworkType::Testnet(TestnetType::Sepolia),
            "goerli" => NetworkType::Testnet(TestnetType::Goerli),
            "mumbai" => NetworkType::Testnet(TestnetType::Mumbai),
            "fuji" => NetworkType::Testnet(TestnetType::Fuji),
            _ => NetworkType::Testnet(TestnetType::Other(network.to_string())),
        };

        let config = Config::new(
            BlockchainConfig {
                chain_type,
                network: network_type,
                port: 8545, // Default port, can be made configurable
                provider_url: provider_url.to_string(),
            },
            WalletConfig {
                public_key: public_key.to_string(),
                private_key: private_key.to_string(),
            },
        );

        self.context_manager.add_config(name, config);
        Ok(())
    }

    pub fn use_configuration(&mut self, name: &str) -> Result<(), String> {
        self.context_manager.set_active_config(name)
    }

    pub async fn initialize_identity_manager(&mut self, contract_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contract_address: Address = contract_addr.parse().expect("Invalid address");
        let config = self.context_manager.get_active_config().ok_or("No active configuration")?;
        let provider = Provider::<Http>::try_from(config.blockchain.provider_url.clone())?;
        let client = Arc::new(provider);
        let wallet = LocalWallet::from_bytes(&hex::decode(&config.wallet.private_key)?)?;
        
        self.identity_manager = Some(IdentityManager::new(contract_address, client, wallet));
        Ok(())
    }

    pub async fn load_identity(&self, did: &str) -> Result<Identity, Box<dyn std::error::Error>> {
        let did_addr:Address = did.parse().expect("Invalid Address");
        self.identity_manager
            .as_ref()
            .ok_or("IdentityManager not initialized")?
            .load_identity(did_addr)
            .await
    }

    pub async fn sign_message(&self, message: &[u8]) -> Result<Signature, Box<dyn std::error::Error>> {
        self.identity_manager
            .as_ref()
            .ok_or("IdentityManager not initialized")?
            .sign_message(message)
            .await
    }

    pub fn verify_signature(&self, message: &[u8], signature: &Signature, signer: Address) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.identity_manager
            .as_ref()
            .ok_or("IdentityManager not initialized")?
            .verify_signature(message, signature, signer))
    }
}

