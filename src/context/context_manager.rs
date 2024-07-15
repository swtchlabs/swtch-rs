// src/context/context_manager.rs

use super::config::{Config, BlockchainConfig, WalletConfig, ChainType, NetworkType};
use std::collections::HashMap;

pub struct ContextManager {
    configs: HashMap<String, Config>,
    active_config: String,
}

impl ContextManager {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
            active_config: String::new(),
        }
    }

    pub fn add_config(&mut self, name: &str, config: Config) {
        self.configs.insert(name.to_string(), config);
        if self.active_config.is_empty() {
            self.active_config = name.to_string();
        }
    }

    pub fn set_active_config(&mut self, name: &str) -> Result<(), String> {
        if self.configs.contains_key(name) {
            self.active_config = name.to_string();
            Ok(())
        } else {
            Err(format!("Configuration '{}' not found", name))
        }
    }

    pub fn get_active_config(&self) -> Option<&Config> {
        self.configs.get(&self.active_config)
    }

    // Helper methods to abstract blockchain details
    pub fn get_current_chain_type(&self) -> Option<&ChainType> {
        self.get_active_config().map(|c| &c.blockchain.chain_type)
    }

    pub fn get_current_network(&self) -> Option<&NetworkType> {
        self.get_active_config().map(|c| &c.blockchain.network)
    }

    pub fn get_current_provider_url(&self) -> Option<&str> {
        self.get_active_config().map(|c| &c.blockchain.provider_url).map(|x| x.as_str())
    }

    pub fn get_current_wallet_public_key(&self) -> Option<&str> {
        self.get_active_config().map(|c| &c.wallet.public_key).map(|x| x.as_str())
    }

    // Add more helper methods as needed
}