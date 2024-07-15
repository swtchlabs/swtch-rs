// tests/common/mod.rs

use swtch_sdk::{SwtchSDK, BlockchainConfig, ChainType, context::{ContextManager, Config}, identity::IdentityManager, NetworkType, TestnetType, WalletConfig};
use ethers::prelude::*;
use ethers::providers::{Provider as EthersProvider, Http, JsonRpcClient};
use ethers::types::{
    transaction::eip2718::TypedTransaction,
    BlockId, TxHash, TransactionReceipt, U64, Bytes, Address,
};
use std::sync::Arc;
use mockall::mock;
use mockall::predicate::*;
use rand::Rng;
use async_trait::async_trait;
use std::fmt;
use std::marker::PhantomData;

use std::fmt::Debug;
use serde::Serialize;
use serde::de::DeserializeOwned;

// Mock Provider
#[derive(Debug, Clone)]
pub struct CustomMockProvider;

#[async_trait]
impl JsonRpcClient for CustomMockProvider {
    type Error = ProviderError;

    async fn request<T: Send + Sync, R>(&self, _method: &str, _params: T) -> Result<R, Self::Error>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        Err(ProviderError::CustomError("Mock provider always returns an error".to_string()))
    }
}

#[async_trait]
impl Middleware for CustomMockProvider {
    type Error = ProviderError;
    type Provider = CustomMockProvider;
    type Inner = CustomMockProvider;

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn default_sender(&self) -> Option<Address> {
        None
    }

    async fn get_balance<T: Into<NameOrAddress> + Send + Sync>(&self, _addr: T, _block: Option<BlockId>) -> Result<U256, Self::Error> {
        Ok(U256::from(100))
    }

    async fn get_block_number(&self) -> Result<U64, Self::Error> {
        Err(ProviderError::CustomError("Mock get_block_number".to_string()))
    }

    async fn send_transaction<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        _tx: T,
        _block: Option<BlockId>,
    ) -> Result<PendingTransaction<'_, Self::Provider>, Self::Error> {
        Err(ProviderError::CustomError("Mock send_transaction".to_string()))
    }

    async fn get_transaction_receipt<T: Send + Sync + Into<TxHash>>(
        &self,
        _tx_hash: T,
    ) -> Result<Option<TransactionReceipt>, Self::Error> {
        Err(ProviderError::CustomError("Mock get_transaction_receipt".to_string()))
    }
    

    async fn call(&self, _tx: &TypedTransaction, _block: Option<BlockId>) -> Result<Bytes, Self::Error> {
        Err(ProviderError::CustomError("Mock call".to_string()))
    }

    // Implement other required methods...
}

// Setup function for SwtchSDK
pub fn setup_sdk() -> SwtchSDK {
    let mut sdk = SwtchSDK::new();
    let mut context_manager = ContextManager::new();
    
    // Add a test configuration
    let config = Config {
        blockchain: BlockchainConfig {
            chain_type: ChainType::Ethereum,
            network: NetworkType::Testnet(TestnetType::Sepolia),
            port: 8545,
            provider_url: "http://localhost:8545".to_string(),
        },
        wallet: WalletConfig {
            public_key: "0x1234567890123456789012345678901234567890".to_string(),
            private_key: "0x1234567890123456789012345678901234567890123456789012345678901234".to_string(),
        },
    };
    
    context_manager.add_config("test", config);
    context_manager.set_active_config("test").unwrap();
    
    sdk
}

// Function to create a mock provider
pub fn mock_provider() -> CustomMockProvider {
    CustomMockProvider
}

// Function to create a local wallet for testing
pub fn create_test_wallet() -> LocalWallet {
    LocalWallet::new(&mut rand::thread_rng())
}

// Function to generate a random Ethereum address
pub fn random_address() -> Address {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 20];
    rng.fill(&mut bytes);
    Address::from(bytes)
}

// Function to create a mock IdentityManager
pub fn mock_identity_manager() -> IdentityManager<CustomMockProvider> {
    let mock_provider = mock_provider();
    let wallet = LocalWallet::new(&mut rand::thread_rng());
    let contract_address = Address::random();
    IdentityManager::new(contract_address, Arc::new(mock_provider), wallet)
}

// Helper function to create a mock transaction receipt
pub fn mock_transaction_receipt() -> TransactionReceipt {
    TransactionReceipt {
        transaction_hash: H256::random(),
        transaction_index: U64::from(0),
        block_hash: Some(H256::random()),
        block_number: Some(U64::from(1)),
        from: random_address(),
        to: Some(random_address()),
        cumulative_gas_used: U256::from(21000),
        gas_used: Some(U256::from(21000)),
        contract_address: None,
        logs: vec![],
        status: Some(U64::from(1)),
        root: None,
        logs_bloom: Bloom::default(),
        transaction_type: None,
        effective_gas_price: Some(U256::from(1000000000)),
        other: Default::default(), // Add this line
    }
}

// Add more utility functions as needed...