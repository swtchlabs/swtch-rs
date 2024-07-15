// src/context/config.rs

pub struct BlockchainConfig {
    pub chain_type: ChainType,
    pub network: NetworkType,
    pub port: u16,
    pub provider_url: String,
    // Add any other necessary blockchain-specific configs
}

pub struct WalletConfig {
    pub public_key: String,
    pub private_key: String,
}

pub struct Config {
    pub blockchain: BlockchainConfig,
    pub wallet: WalletConfig,
    // Add any other global configs
}

impl Config {
    pub fn new(blockchain: BlockchainConfig, wallet: WalletConfig) -> Self {
        Self { blockchain, wallet }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainType {
    Ethereum,
    Polygon,
    Avalanche,
    // Add other supported chains
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkType {
    Mainnet,
    Testnet(TestnetType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestnetType {
    // Ethereum testnets
    Sepolia,
    Goerli,
    // Polygon testnets
    Mumbai,
    // Avalanche testnets
    Fuji,
    // Add other testnets as needed
    Other(String), // For flexibility
}