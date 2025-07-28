       // E:\Rust\Projects\hello-world\contracts\counter_contract\src\lib.rs

       #![no_std] // Don't link the standard library, as Wasm environments are restricted
       use soroban_sdk::{contract, contractimpl, Env, Symbol, storage::Instance, symbol_short};

       // Define a Symbol for our storage key. Symbols are efficient on-chain strings.
       const COUNT_KEY: Symbol = symbol_short!("COUNT"); // Using symbol_short! for efficiency

       // Define the contract struct. This is purely for organizational purposes in Rust.
       // The contract implementation methods will be associated with this struct.
       #[contract]
       pub struct CounterContract;

       // Implement the contract logic within an `impl` block for our contract struct.
       // The `#[contractimpl]` attribute exposes these methods as callable functions on the smart contract.
       #[contractimpl]
       impl CounterContract {
           /// Initializes the counter to a given value.
           /// This method can only be called once per contract instance.
           pub fn initialize(env: Env, count: u32) {
               // Get the instance storage. Instance storage lives for the lifetime of the contract.
               let storage = env.storage().instance();

               // Check if the contract is already initialized.
               // If the `COUNT_KEY` already exists, it means `initialize` has been called before.
               if storage.has(&COUNT_KEY) {
                   panic!("Contract already initialized");
               }

               // Put the initial count into the instance storage.
               storage.set(&COUNT_KEY, &count);

               // Set a 7-day TTL (Time To Live) for the instance storage.
               // This ensures the contract state doesn't get archived by the network due to inactivity,
               // but it will be extended on every interaction.
               // The values here should be in seconds for `extend_ttl` as per `soroban_sdk` documentation for `Instance` storage.
               // (7 days * 24 hours/day * 60 minutes/hour * 60 seconds/minute) = 604800 seconds.
               storage.extend_ttl(604800, 604800);
           }

           /// Retrieves the current value of the counter.
           pub fn get_count(env: Env) -> u32 {
               // Get the instance storage.
               let storage = env.storage().instance();

               // Get the count from storage. If it doesn't exist (e.g., not initialized), default to 0.
               // `.unwrap_or(0)` is a common Rust idiom for Option types.
               storage.get(&COUNT_KEY).unwrap_or(0)
           }

           /// Increments the counter by a specified value.
           pub fn increment(env: Env, delta: u32) -> u32 {
               // Get the instance storage, allowing mutable access.
               let storage = env.storage().instance();

               // Get the current count, defaulting to 0 if not found.
               let mut count: u32 = storage.get(&COUNT_KEY).unwrap_or(0);

               // Increment the count.
               count += delta;

               // Store the new count back into storage.
               storage.set(&COUNT_KEY, &count);

               // Extend the TTL on every interaction to keep the contract alive.
               storage.extend_ttl(604800, 604800); // 7 days

               // Return the new count.
               count
           }

           /// Decrements the counter by a specified value.
           /// Panics if the count would go below zero.
           pub fn decrement(env: Env, delta: u32) -> u32 {
               let storage = env.storage().instance();
               let mut count: u32 = storage.get(&COUNT_KEY).unwrap_or(0);

               if count < delta {
                   panic!("Count cannot go below zero");
               }

               count -= delta;
               storage.set(&COUNT_KEY, &count);
               storage.extend_ttl(604800, 604800); // 7 days
               count
           }
       }