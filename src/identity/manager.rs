// src/identity/manager.rs
use super::models::Identity;
use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::signers::{LocalWallet, Signer};
use std::sync::Arc;

abigen!(
    IdentityManagerContract,
    r#"[
        function registerIdentity(address did, address owner, string memory documentHash) external
        function setDIDDocument(address did, string memory documentHash) external
        function addDelegate(address did, address delegate) external
        function removeDelegate(address did, address delegate) external
        function isOwnerOrDelegate(address did, address user) external view returns (bool)
        function addAttestation(address did, string memory issuer, string memory claim) external
        function getAttestations(address did) external view returns (string issuer, string claim, uint256 issuedAt)
        function verifyAttestation(address did, string memory issuer, string memory claim) external view returns (bool)
        function identities(address) external view returns (address owner, address claimsContract, string didDocument)
    ]"#,
);

pub struct IdentityManager<M: Middleware> {
    pub contract: IdentityManagerContract<M>,
    pub wallet: LocalWallet,
}

impl<M: Middleware + 'static> IdentityManager<M> {
    pub fn new(address: Address, client: Arc<M>, wallet: LocalWallet) -> Self {
        let contract = IdentityManagerContract::new(address, Arc::clone(&client));
        Self { contract, wallet }
    }

    pub async fn load_identity(&self, did: Address) -> Result<Identity, Box<dyn std::error::Error>> {
        let (owner, claims_contract, did_document) = self.contract.identities(did).call().await?;
        Ok(Identity {
            address: did,
            owner,
            claims_contract,
            did_document,
        })
    }

    pub async fn register_identity(&self, did: Address, owner: Address, document_hash: String) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
        let tx = self.contract.register_identity(did, owner, document_hash);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn set_did_document(&self, did: Address, document_hash: String) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
        let tx = self.contract.set_did_document(did, document_hash);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn add_delegate(&self, did: Address, delegate: Address) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
        let tx = self.contract.add_delegate(did, delegate);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn remove_delegate(&self, did: Address, delegate: Address) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
        let tx = self.contract.remove_delegate(did, delegate);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn is_owner_or_delegate(&self, did: Address, user: Address) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.contract.is_owner_or_delegate(did, user).call().await?)
    }

    pub async fn add_attestation(&self, did: Address, issuer: String, claim: String) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
        let tx = self.contract.add_attestation(did, issuer, claim);
        let pending_tx = tx.send().await?;
        Ok(pending_tx.await?.expect("Transaction failed"))
    }

    pub async fn get_attestations(&self, did: Address) -> Result<Vec<(String, String, U256)>, Box<dyn std::error::Error>> {
        let (issuer, claim, issued_at) = self.contract.get_attestations(did).call().await?;
        Ok(vec![(issuer, claim, issued_at)])
    }

    pub async fn verify_attestation(&self, did: Address, issuer: String, claim: String) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.contract.verify_attestation(did, issuer, claim).call().await?)
    }

    pub async fn sign_message(&self, message: &[u8]) -> Result<Signature, Box<dyn std::error::Error>> {
        Ok(self.wallet.sign_message(message).await?)
    }

    pub fn verify_signature(&self, message: &[u8], signature: &Signature, signer: Address) -> bool {
        signature.verify(message, signer).is_ok()
    }
}