// E:\Rust\Projects\wallet\utilities\stellar_wallet_tools\src\lib.rs

use stellar_sdk::{Keypair, Network, TransactionBuilder, StellarSdkError, Asset, PaymentOperation};
use stellar_sdk::sep::mnemonic::{Mnemonic, Language};
use rand::thread_rng;
use std::str::FromStr; // Required for parsing asset codes

/// Represents a generated Stellar key pair.
pub struct StellarKeyPair {
    pub mnemonic_phrase: String,
    pub public_key: String,
    pub secret_key: String,
}

/// A utility struct for performing Stellar wallet-related operations.
/// This struct encapsulates key generation and transaction building functionalities.
pub struct StellarWalletTools;

impl StellarWalletTools {
    /// Generates a new Stellar key pair including a mnemonic phrase,
    /// public key (G...), and secret key (S...).
    ///
    /// This is an associated function (like a static method in other languages)
    /// as it doesn't require an instance of `StellarWalletTools` to operate.
    ///
    /// # Returns
    /// A `Result` containing `StellarKeyPair` on success, or an error if generation fails.
    pub fn generate_key_pair() -> Result<StellarKeyPair, Box<dyn std::error::Error>> {
        // 1. Generate a new random mnemonic phrase (seed phrase)
        // This is the human-readable backup of your key.
        let mnemonic = Mnemonic::generate(&mut thread_rng(), Language::English)?;

        // 2. Derive the Keypair from the mnemonic phrase.
        // The `from_mnemonic_phrase` function uses the mnemonic to deterministically
        // generate the public and secret keys. The `None` argument means no passphrase
        // extension is used, which is typical for standard BIP39 mnemonics.
        let keypair = Keypair::from_mnemonic_phrase(&mnemonic.phrase(), None)?;

        // 3. Get the Public Key (starts with 'G')
        // This is your account address, safe to share.
        let public_key = keypair.public_key();

        // 4. Get the Secret Key (starts with 'S')
        // This is your private key, used to sign transactions. KEEP THIS ABSOLUTELY SECRET!
        let secret_key = keypair.secret();

        Ok(StellarKeyPair {
            mnemonic_phrase: mnemonic.phrase().to_string(),
            public_key: public_key.to_string(),
            secret_key: secret_key.to_string(),
        })
    }

    /// Creates a payment transaction.
    ///
    /// This is also an associated function, as transaction creation typically
    /// doesn't require maintaining state within the `StellarWalletTools` struct itself.
    ///
    /// # Arguments
    /// * `source_secret_key` - The secret key of the account sending the payment.
    /// * `destination_public_key` - The public key of the account receiving the payment.
    /// * `amount` - The amount to send (as a string, e.g., "10.5").
    /// * `asset_code` - The asset code (e.g., "XLM", "USD").
    /// * `asset_issuer_public_key` - The issuer's public key for non-native assets. Use `None` for XLM.
    /// * `network_passphrase` - The Stellar network passphrase (e.g., for Testnet).
    ///
    /// # Returns
    /// A `Result` containing the base64-encoded transaction XDR string on success,
    /// or a `StellarSdkError` if transaction creation fails.
    pub async fn create_payment_transaction(
        source_secret_key: &str,
        destination_public_key: &str,
        amount: &str,
        asset_code: &str,
        asset_issuer_public_key: Option<&str>,
        network_passphrase: &str,
    ) -> Result<String, StellarSdkError> {
        let source_keypair = Keypair::from_secret(source_secret_key)?;
        let destination_account_id = destination_public_key.to_string();

        // Determine the asset to send
        let asset = if asset_code == "XLM" {
            Asset::native()
        } else {
            let issuer_keypair = Keypair::from_public_key(
                asset_issuer_public_key.ok_or(StellarSdkError::InvalidAsset("Issuer key required for non-native asset".to_string()))?
            )?;
            Asset::new(
                asset_code.to_string(),
                issuer_keypair.public_key().to_string(),
            )?
        };

        // Load the source account to get its sequence number
        let horizon_client = stellar_sdk::horizon::HorizonClient::new("https://horizon-testnet.stellar.org"); // Assuming Testnet for now
        let source_account = horizon_client.get_account(&source_keypair.public_key()).await?;

        // Create the payment operation
        let operation = PaymentOperation::new(
            destination_account_id,
            asset,
            amount.to_string(),
        );

        // Build the transaction
        let transaction = TransactionBuilder::new(
            source_account,
            Network::from_str(network_passphrase)?,
        )
        .add_operation(operation)
        .build();

        // Sign the transaction
        let signed_transaction = transaction.sign(&source_keypair)?;

        // Return the base64-encoded XDR
        Ok(signed_transaction.to_xdr_base64())
    }
}
