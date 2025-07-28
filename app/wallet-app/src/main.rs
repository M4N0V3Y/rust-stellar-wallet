// Import the StellarWalletTools struct from your utility library
use stellar_wallet_tools_lib::StellarWalletTools;
use tokio::main; // For async main function
use std::io::{self, Write}; // For flush

#[main]
async fn main() {
    println!("--- Wallet Pilot Stellar Project CLI ---"); // Updated title

    loop {
        println!("\nChoose an action:");
        println!("1. Generate a new Stellar Key Pair");
        println!("2. Create a Testnet Payment Transaction (Example)");
        println!("3. Exit");
        print!("Enter choice (1, 2, or 3): ");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure prompt is displayed immediately

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                // Call the associated function on the StellarWalletTools struct
                match StellarWalletTools::generate_key_pair() {
                    Ok(key_pair) => {
                        println!("\nGenerated Seed Phrase (Mnemonic):");
                        println!("\"{}\"", key_pair.mnemonic_phrase);

                        println!("\nPublic Key (G...):");
                        println!("\"{}\"", key_pair.public_key);

                        println!("\nSecret Key (S...):");
                        println!("\"{}\"", key_pair.secret_key);

                        println!("\n!!! IMPORTANT SECURITY WARNING !!!");
                        println!("----------------------------------");
                        println!("NEVER SHARE YOUR SECRET KEY OR SEED PHRASE WITH ANYONE.");
                        println!("Anyone who has these can access and control your funds on the Stellar network.");
                        println!("For real funds, consider using hardware wallets or other secure key management solutions.");
                        println!("This utility is for development and testing purposes only.");
                        println!("----------------------------------");
                    }
                    Err(e) => {
                        eprintln!("Error generating key pair: {}", e);
                    }
                }
            }
            "2" => {
                println!("\n--- Create Testnet Payment Transaction ---");
                print!("Enter Source Secret Key (S...): ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut source_secret_key = String::new();
                io::stdin().read_line(&mut source_secret_key).expect("Failed to read source secret key");
                let source_secret_key = source_secret_key.trim();

                print!("Enter Destination Public Key (G...): ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut destination_public_key = String::new();
                io::stdin().read_line(&mut destination_public_key).expect("Failed to read destination public key");
                let destination_public_key = destination_public_key.trim();

                print!("Enter Amount (e.g., 10.5): ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read amount");
                let amount = amount.trim();

                print!("Enter Asset Code (e.g., XLM, USD): ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut asset_code = String::new();
                io::stdin().read_line(&mut asset_code).expect("Failed to read asset code");
                let asset_code = asset_code.trim().to_uppercase();

                let mut asset_issuer_public_key: Option<String> = None;
                if asset_code != "XLM" {
                    print!("Enter Asset Issuer Public Key (G...): ");
                    io::stdout().flush().expect("Failed to flush stdout");
                    let mut issuer = String::new();
                    io::stdin().read_line(&mut issuer).expect("Failed to read issuer public key");
                    asset_issuer_public_key = Some(issuer.trim().to_string());
                }

                let network_passphrase = "Test SDF Network ; September 2015"; // Hardcoded for Testnet

                // Call the associated function on the StellarWalletTools struct
                match StellarWalletTools::create_payment_transaction(
                    source_secret_key,
                    destination_public_key,
                    amount,
                    &asset_code,
                    asset_issuer_public_key.as_deref(), // Convert Option<String> to Option<&str>
                    network_passphrase,
                ).await {
                    Ok(xdr) => {
                        println!("\nTransaction created successfully!");
                        println!("Signed Transaction XDR (Base64):");
                        println!("{}", xdr);
                        println!("\nUse this XDR to submit the transaction to a Stellar Horizon endpoint.");
                        println!("Example: https://laboratory.stellar.org/#txbuilder?xdr={}", xdr);
                    }
                    Err(e) => {
                        eprintln!("Error creating transaction: {}", e);
                    }
                }
            }
            "3" => {
                println!("Exiting application. Goodbye!");
                break; // Exit the loop
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, or 3.");
            }
        }
    }
}