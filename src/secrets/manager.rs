// src/secrets/manager.rs

use ethers::prelude::*;
use std::sync::Arc;

use super::space::SecretsSpace;

abigen!(
    SecretsManagerContract,
    r#"[
        function getFee() external view returns (uint256)
        function getSpace(address userDID) public view returns (address)
        function getSubSpaces(address userDID) public view returns (address[])
        function addSpace(address userDID) external
        function addSubSpace(address userDID, address subUserDID) external
        function disableSpace(address userDID) external
    ]"#,
);

pub struct SecretsManager<M: Middleware> {
    secrets_manager: SecretsManagerContract<M>,
    client: Arc<M>,
}

impl<M: Middleware + 'static> SecretsManager<M> {
    pub fn new(address: Address, client: Arc<M>) -> Self {
        let secrets_manager = SecretsManagerContract::new(address, Arc::clone(&client));
        Self { secrets_manager, client }
    }

    pub async fn get_fee(&self) -> Result<U256, ContractError<M>> {
        self.secrets_manager.get_fee().call().await
    }

    pub async fn get_space(&self, user_did: Address) -> Result<Address, ContractError<M>> {
        self.secrets_manager.get_space(user_did).call().await
    }

    pub async fn get_sub_spaces(&self, user_did: Address) -> Result<Vec<Address>, ContractError<M>> {
        self.secrets_manager.get_sub_spaces(user_did).call().await
    }

    pub async fn add_space(&self, user_did: Address) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.secrets_manager.add_space(user_did);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn add_sub_space(&self, user_did: Address, sub_user_did: Address) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.secrets_manager.add_sub_space(user_did, sub_user_did);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn disable_space(&self, user_did: Address) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.secrets_manager.disable_space(user_did);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn get_secrets_space(&self, space_address: Address) -> SecretsSpace<M> {
        SecretsSpace::new(space_address, Arc::clone(&self.client))
    }
}