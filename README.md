# SWTCH Protocol SDK for Rust 

## Build & Run

### Install
```sh
cargo install
```

### Build
```sh
cargo build
```

## Test

### Run all tests
```sh
cargo test
```

### Run only SDK tests
```sh
cargo test --test sdk_tests
```

### Run tests with names containing 'identity'
```sh
cargo test identity
```

## Core Module

### SwtchSDK
Manages the overall SDK operations and configuration.

#### Features
- Add Configuration: Method to add blockchain configurations.
- Use Configuration: Method to set the active configuration.
- Initialize Identity Manager: Method to set up the Identity Manager with the current configuration.

### Context Manager
Ensures the correct blockchain configuration is used for each operation.

#### Features
- Add Config: Method to add a new blockchain configuration.
- Set Active Config: Method to set the current active configuration.
- Get Active Config: Method to retrieve the current active configuration.

### Identity Manager
Manages interactions with the IdentityManager smart contract.

#### Features
- Load Identity: Method to load existing identities from the blockchain.
- Register Identity: Method to register new identities on the blockchain.
- Set DID Document: Method to update the DID document for an identity.
- Add/Remove Delegate: Methods to manage delegates for an identity.
- Add Attestation: Method to add attestations to an identity.
- Get Attestations: Method to retrieve attestations for an identity.
- Verify Attestation: Method to verify a specific attestation.
- Sign Message: Method to sign a message using the wallet.
- Verify Signature: Method to verify a signature.

### Identity
Represents a decentralized identity in the system.

#### Attributes
- Address: The blockchain address of the identity.
- Owner: The owner's address of the identity.
- Claims Contract: The address of the associated claims contract.
- DID Document: The DID document associated with the identity.