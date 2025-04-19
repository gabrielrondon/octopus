#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, String, Symbol};

/// Octopus: CIDMapper - IPCM Contract Implementation
/// Manages mappings between token IDs and their IPFS CIDs
/// Emits events for all mapping updates to create a verifiable history

// Define storage keys
const OWNER_KEY: Symbol = Symbol::short("OWNER");
const MAPPING_KEY: Symbol = Symbol::short("MAP");

// Define events
const UPDATE_MAPPING_EVENT: Symbol = Symbol::short("UPDATE_MAP");

#[contract]
pub struct OctopusIPCMContract;

#[contractimpl]
impl OctopusIPCMContract {
    /// Initialize the contract with an owner
    pub fn initialize(env: Env, owner: Address) {
        // Ensure the contract is not already initialized
        if env.storage().instance().has(&OWNER_KEY) {
            panic!("Contract already initialized");
        }
        
        // Set the contract owner
        env.storage().instance().set(&OWNER_KEY, &owner);
    }
    
    /// Update a token's CID mapping - only owner can call
    pub fn update_mapping(env: Env, caller: Address, token_id: String, cid: String) {
        // Check if caller is authorized
        Self::require_auth(&env, &caller);
        
        // Get current mapping if it exists
        let mapping_key = Self::get_mapping_key(&token_id);
        let old_cid = if env.storage().persistent().has(&mapping_key) {
            env.storage().persistent().get::<_, String>(&mapping_key).unwrap_or(String::from_str(&env, ""))
        } else {
            String::from_str(&env, "")
        };
        
        // Update the mapping
        env.storage().persistent().set(&mapping_key, &cid);
        
        // Emit an event for the mapping update
        env.events().publish(
            (UPDATE_MAPPING_EVENT, token_id.clone()),
            (token_id, old_cid, cid, caller),
        );
    }
    
    /// Get a token's current CID mapping
    pub fn get_mapping(env: Env, token_id: String) -> String {
        let mapping_key = Self::get_mapping_key(&token_id);
        if env.storage().persistent().has(&mapping_key) {
            env.storage().persistent().get(&mapping_key).unwrap()
        } else {
            String::from_str(&env, "")
        }
    }
    
    /// Transfer ownership of the contract
    pub fn transfer_ownership(env: Env, caller: Address, new_owner: Address) {
        // Verify current owner
        Self::require_owner(&env, &caller);
        
        // Update owner
        env.storage().instance().set(&OWNER_KEY, &new_owner);
        
        // Emit ownership transfer event
        env.events().publish(
            Symbol::short("TRANSFER_OWNER"),
            (caller, new_owner),
        );
    }
    
    // Helper functions
    
    /// Check if the caller is the contract owner
    fn require_owner(env: &Env, caller: &Address) {
        let owner: Address = env.storage().instance().get(&OWNER_KEY).unwrap();
        if *caller != owner {
            panic!("Caller is not the contract owner");
        }
        caller.require_auth();
    }
    
    /// Check if the caller is authorized (for now just the owner)
    fn require_auth(env: &Env, caller: &Address) {
        Self::require_owner(env, caller);
    }
    
    /// Create a unique storage key for each token mapping
    fn get_mapping_key(token_id: &String) -> Symbol {
        // In a production contract, you would use a more sophisticated approach
        // to generate unique storage keys for different token IDs
        Symbol::new(token_id.to_string())
    }
}

// Tests for the IPCM contract
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};
    use soroban_sdk::{symbol_short, vec, Env};

    #[test]
    fn test_ipcm_contract() {
        let env = Env::default();
        let contract_id = env.register_contract(None, OctopusIPCMContract);
        
        let owner = Address::random(&env);
        let user = Address::random(&env);
        
        let client = OctopusIPCMContractClient::new(&env, &contract_id);
        
        // Initialize the contract
        client.initialize(&owner);
        
        // Test updating a mapping
        let token_id = String::from_str(&env, "token123");
        let initial_cid = String::from_str(&env, "QmInitialCID");
        client.update_mapping(&owner, &token_id, &initial_cid);
        
        // Test retrieving a mapping
        let retrieved_cid = client.get_mapping(&token_id);
        assert_eq!(retrieved_cid, initial_cid);
        
        // Update the mapping
        let new_cid = String::from_str(&env, "QmNewCID");
        client.update_mapping(&owner, &token_id, &new_cid);
        
        // Verify the update
        let updated_cid = client.get_mapping(&token_id);
        assert_eq!(updated_cid, new_cid);
        
        // Transfer ownership
        let new_owner = Address::random(&env);
        client.transfer_ownership(&owner, &new_owner);
        
        // New owner should be able to update mappings
        let final_cid = String::from_str(&env, "QmFinalCID");
        client.update_mapping(&new_owner, &token_id, &final_cid);
        
        // Old owner should not be able to update mappings
        let result = std::panic::catch_unwind(|| {
            client.update_mapping(&owner, &token_id, &initial_cid);
        });
        assert!(result.is_err());
    }
}