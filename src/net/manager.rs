// src/network/manager.rs

use ethers::prelude::*;
use std::sync::Arc;

abigen!(
    NetworkManagerContract,
    r#"[
        function addNetworkService(address provider, string memory serviceDetails) external
        function getNetworkService(address provider) external view returns(tuple(address owner, string serviceDetails, bool isActive))
        function updateNetworkService(address provider, string memory newServiceDetails) external
        function removeNetworkService(address provider) external
        function isServiceProvider(address provider) external view returns (bool)
        function getServiceProviders() external view returns (address[] memory)
    ]"#,
);

pub struct NetworkManager<M: Middleware> {
    contract: NetworkManagerContract<M>,
    client: Arc<M>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkService {
    pub owner: Address,
    pub service_details: String,
    pub is_active: bool,
}

impl<M: Middleware + 'static> NetworkManager<M> {
    
    pub fn new(address: Address, client: Arc<M>) -> Self {
        let contract = NetworkManagerContract::new(address, Arc::clone(&client));
        Self { contract, client }
    }

    pub async fn add_network_service(&self, provider: Address, service_details: String) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.add_network_service(provider, service_details);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn get_network_service(&self, provider: Address) -> Result<NetworkService, ContractError<M>> {
        let (owner, service_details, is_active) = self.contract.get_network_service(provider).call().await?;
        Ok(NetworkService {
            owner,
            service_details,
            is_active,
        })
    }

    pub async fn update_network_service(&self, provider: Address, new_service_details: String) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.update_network_service(provider, new_service_details);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn remove_network_service(&self, provider: Address) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.remove_network_service(provider);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn is_service_provider(&self, provider: Address) -> Result<bool, ContractError<M>> {
        self.contract.is_service_provider(provider).call().await
    }

    pub async fn get_service_providers(&self) -> Result<Vec<Address>, ContractError<M>> {
        self.contract.get_service_providers().call().await
    }
}