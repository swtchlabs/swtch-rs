// src/secrets/space.rs

use ethers::prelude::*;
use ethers::types::Bytes;
use std::sync::Arc;

abigen!(
    SecretsSpaceContract,
    r#"[
        function getFee() public view returns(uint256)
        function adjustFees(uint256 newFee) public
        function addSecret(bytes memory identifier, bytes memory secretValue) external payable
        function feesCollected() public view returns(uint256)
        function withdrawFees(address payable recipient, uint256 amount) public
        function getSecret(bytes memory identifier) public returns (bytes memory)
        function authorizeDelegate(address delegate, bytes memory identifier) public
        function revokeDelegate(address delegate, bytes memory identifier) public
        function deleteSecret(bytes memory identifier) public
    ]"#,
);

pub struct SecretsSpace<M: Middleware> {
    secrets_space: SecretsSpaceContract<M>,
}

impl<M: Middleware + 'static> SecretsSpace<M> {
    pub fn new(address: Address, client: Arc<M>) -> Self {
        let secrets_space = SecretsSpaceContract::new(address, client);
        Self { secrets_space }
    }

    pub async fn get_fee(&self) -> Result<U256, ContractError<M>> {
        self.secrets_space.get_fee().call().await
    }

    pub async fn adjust_fees(&self, new_fee: U256) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.secrets_space.adjust_fees(new_fee);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn add_secret(&self, identifier: Bytes, secret_value: Bytes, value: U256) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.secrets_space.add_secret(identifier, secret_value).value(value);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn fees_collected(&self) -> Result<U256, ContractError<M>> {
        let fees = self.secrets_space.fees_collected().call().await?;
        Ok(fees)
    }

    pub async fn withdraw_fees(&self, recipient: Address, amount: U256) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.secrets_space.withdraw_fees(recipient, amount);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn get_secret(&self, identifier: Bytes) -> Result<Bytes, ContractError<M>> {
        let secret = self.secrets_space.get_secret(identifier).call().await?;
        Ok(secret)
    }

    pub async fn authorize_delegate(&self, delegate: Address, identifier: Bytes) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.secrets_space.authorize_delegate(delegate, identifier);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn revoke_delegate(&self, delegate: Address, identifier: Bytes) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.secrets_space.revoke_delegate(delegate, identifier);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn delete_secret(&self, identifier: Bytes) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.secrets_space.delete_secret(identifier);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }
}