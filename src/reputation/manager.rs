// src/reputation/manager.rs
use super::models::Reputation;
use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::signers::{LocalWallet, Signer};
use std::sync::Arc;

abigen!(
    ReputationManagerContract,
    r#"[
        function updateScore(address did, bool isProducer, bytes32 actionType, bool success) public
        function setActionWeight(address did, bytes32 actionType, uint256 weight) public
        function updateProductScore(address did, bytes32 productHash, uint256 newScore) public
        function getCompleteProfile(address did) public view returns (uint256, uint256, uint256)
        function getProductScore(address did, bytes32 productHash) public view returns (uint256)
        function initiateEscrow() external payable
        function releaseEscrow() external
        function refundEscrow() external
        function initiateERC20Escrow(uint256 amount) external
        function releaseERC20Escrow() external
        function refundERC20Escrow() external
        function initiateERC721Escrow() external
        function releaseERC721Escrow() external
        function refundERC721Escrow() external
        function setIdentityManager(address _newIdentityManager) external
        function setEthEscrow(address _newEthEscrow) external
        function setERC20Escrow(address _newERC20Escrow) external
        function setERC721Escrow(address _newERC721Escrow) external
    ]"#,
);

pub struct ReputationManager<M: Middleware> {
    pub contract: ReputationManagerContract<M>,
    pub wallet: LocalWallet,
}

impl<M: Middleware + 'static> ReputationManager<M> {
    
    pub fn new(address: Address, client: Arc<M>, wallet: LocalWallet) -> Self {
        let contract = ReputationManagerContract::new(address, Arc::clone(&client));
        Self { contract, wallet }
    }
    
    pub async fn update_score(&self, did: Address, is_producer: bool, action_type: [u8; 32], success: bool) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.update_score(did, is_producer, action_type.into(), success);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn set_action_weight(&self, did: Address, action_type: [u8; 32], weight: U256) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.set_action_weight(did, action_type.into(), weight);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn update_product_score(&self, did: Address, product_hash: [u8; 32], new_score: U256) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.update_product_score(did, product_hash.into(), new_score);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn get_complete_profile(&self, did: Address) -> Result<(U256, U256, U256), ContractError<M>> {
        let (consumer_score, participant_score, escrow_balance) = self.contract.get_complete_profile(did).call().await?;
        Ok((consumer_score, participant_score, escrow_balance))
    }

    pub async fn get_product_score(&self, did: Address, product_hash: [u8; 32]) -> Result<U256, ContractError<M>> {
        let product_score = self.contract.get_product_score(did, product_hash.into()).call().await?;
        Ok(product_score)
    }

    pub async fn initiate_escrow(&self, value: U256) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.initiate_escrow().value(value);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn release_escrow(&self) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.release_escrow();
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn refund_escrow(&self) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.refund_escrow();
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn initiate_erc20_escrow(&self, amount: U256) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.initiate_erc20_escrow(amount);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn release_erc20_escrow(&self) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.release_erc20_escrow();
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn refund_erc20_escrow(&self) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.refund_erc20_escrow();
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn initiate_erc721_escrow(&self) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.initiate_erc721_escrow();
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn release_erc721_escrow(&self) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.release_erc721_escrow();
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn refund_erc721_escrow(&self) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.refund_erc721_escrow();
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn set_identity_manager(&self, new_identity_manager: Address) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.set_identity_manager(new_identity_manager);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn set_eth_escrow(&self, new_eth_escrow: Address) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.set_eth_escrow(new_eth_escrow);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn set_erc20_escrow(&self, new_erc20_escrow: Address) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.set_erc20_escrow(new_erc20_escrow);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn set_erc721_escrow(&self, new_erc721_escrow: Address) -> Result<TransactionReceipt, ContractError<M>> {
        let tx = self.contract.set_erc721_escrow(new_erc721_escrow);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }
}