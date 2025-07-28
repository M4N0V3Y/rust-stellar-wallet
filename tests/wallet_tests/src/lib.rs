// E:\Rust\Projects\hello-world\tests\counter_contract_tests\src\lib.rs
// Import the CounterContract and necessary Soroban SDK components for testing.
use counter_contract::CounterContractClient; // Import the generated client for your contract
use stellar_wallet_tools::StellarWalletTools; // Import Stellar wallet tools for testing its functionalities
use soroban_sdk::{Env, testutils::EnvExt, symbol_short};

// The `#[test]` attribute marks a function as a test.
// These are integration tests that interact with the contract client.

// The `#[test]` attribute marks a function as a test.
// These are integration tests that interact with the contract client.

#[test]
fn test_initialize_and_get() {
    // Create a default Soroban environment for testing.
    let env = Env::default();
    // Register the CounterContract in the test environment.
    // `None` means the contract ID will be automatically generated.
    let contract_id = env.register_contract(None, counter_contract::CounterContract);
    // Create a client to interact with the deployed contract in the test environment.
    let client = CounterContractClient::new(&env, &contract_id);

    // Initialize the counter to 0.
    client.initialize(&0);
    // Assert that the count is now 0.
    assert_eq!(client.get_count(), 0);

    // Test re-initialization: it should panic as the contract is already initialized.
    let res = std::panic::catch_unwind(|| {
        client.initialize(&10);
    });
    // Assert that the panic occurred.
    assert!(res.is_err());
}

#[test]
fn test_increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, counter_contract::CounterContract);
    let client = CounterContractClient::new(&env, &contract_id);

    // Initialize the counter to 0.
    client.initialize(&0);
    // Increment by 5 and assert the new count.
    assert_eq!(client.increment(&5), 5);
    // Get the count again to confirm it's persisted.
    assert_eq!(client.get_count(), 5);
    // Increment by 10 and assert the new count.
    assert_eq!(client.increment(&10), 15);
    assert_eq!(client.get_count(), 15);
}

#[test]
fn test_decrement() {
    let env = Env::default();
    let contract_id = env.register_contract(None, counter_contract::CounterContract);
    let client = CounterContractClient::new(&env, &contract_id);

    // Initialize the counter to 20.
    client.initialize(&20);
    // Decrement by 5 and assert the new count.
    assert_eq!(client.decrement(&5), 15);
    assert_eq!(client.get_count(), 15);

    // Test decrement below zero: it should panic.
    let res = std::panic::catch_unwind(|| {
        client.decrement(&20); // Current count is 15, trying to decrement by 20
    });
    assert!(res.is_err());
}

// --- Tests for StellarWalletTools ---

#[test]
fn test_generate_key_pair() {
    // Attempt to generate a key pair
    let key_pair_result = StellarWalletTools::generate_key_pair();

    // Assert that the key pair generation was successful
    assert!(key_pair_result.is_ok(), "Key pair generation failed: {:?}", key_pair_result.err());

    let key_pair = key_pair_result.unwrap();

    // Basic checks on generated keys
    assert!(!key_pair.mnemonic_phrase.is_empty(), "Mnemonic phrase should not be empty");
    assert!(key_pair.public_key.starts_with("G"), "Public key should start with 'G'");
    assert!(key_pair.secret_key.starts_with("S"), "Secret key should start with 'S'");

    // You could add more sophisticated validation here,
    // e.g., attempting to derive the keypair from the mnemonic again
    // and ensuring it matches the generated public/secret keys.
}

#[tokio::test] // Use tokio::test for async functions
async fn test_create_payment_transaction() {
    // Note: This is an integration test that attempts to interact with Horizon.
    // It requires a live Testnet Horizon instance and valid, funded keys.
    // For true unit testing, you might mock the Horizon client.

    // Use a test key pair (replace with actual funded testnet keys for a real test)
    // For demonstration, we'll generate one, but it won't be funded on Testnet.
    // A real test would require pre-funded accounts.
    let source_key_pair_result = StellarWalletTools::generate_key_pair();
    assert!(source_key_pair_result.is_ok());
    let source_key_pair = source_key_pair_result.unwrap();

    let dest_key_pair_result = StellarWalletTools::generate_key_pair();
    assert!(dest_key_pair_result.is_ok());
    let dest_key_pair = dest_key_pair_result.unwrap();

    let source_secret_key = &source_key_pair.secret_key;
    let destination_public_key = &dest_key_pair.public_key;
    let amount = "10";
    let asset_code = "XLM";
    let asset_issuer_public_key = None; // For XLM
    let network_passphrase = "Test SDF Network ; September 2015";

    // Attempt to create a payment transaction
    let transaction_result = StellarWalletTools::create_payment_transaction(
        source_secret_key,
        destination_public_key,
        amount,
        asset_code,
        asset_issuer_public_key,
        network_passphrase,
    ).await; // Await the async function

    // Assert that the transaction creation was successful
    // Note: This test will likely fail if the source_secret_key is not funded
    // on the Testnet, as `get_account` will return an error.
    // For a robust test, you'd need to mock the Horizon client or ensure funding.
    assert!(transaction_result.is_ok(), "Transaction creation failed: {:?}", transaction_result.err());

    let xdr = transaction_result.unwrap();
    assert!(!xdr.is_empty(), "Transaction XDR should not be empty");
    // You could add more assertions here, e.g., decoding the XDR and verifying contents.
}

       