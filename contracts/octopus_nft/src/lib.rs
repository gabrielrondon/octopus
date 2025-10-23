#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String, Symbol, Vec};

/// Octopus: CIDMapper - NFT Contract Implementation
/// Handles token minting, transfers, and ownership
/// Stores references to IPCM keys for each token

// Define storage keys
const ADMIN_KEY: Symbol = Symbol::short("ADMIN");
const IPCM_CONTRACT_KEY: Symbol = Symbol::short("IPCM");
const TOKENS_KEY: Symbol = Symbol::short("TOKENS");
const OWNERS_KEY: Symbol = Symbol::short("OWNERS");
const IPCM_REF_KEY: Symbol = Symbol::short("IPCMREF");

// Define events
const MINT_EVENT: Symbol = Symbol::short("MINT");
const TRANSFER_EVENT: Symbol = Symbol::short("TRANSFER");
const BURN_EVENT: Symbol = Symbol::short("BURN");

#[contract]
pub struct OctopusNFTContract;

#[contractimpl]
impl OctopusNFTContract {
    /// Initialize the contract with an admin and IPCM contract address
    pub fn initialize(env: Env, admin: Address, ipcm_contract: Address) {
        // Ensure the contract is not already initialized
        if env.storage().instance().has(&ADMIN_KEY) {
            panic!("Contract already initialized");
        }
        
        // Set the contract administrator
        env.storage().instance().set(&ADMIN_KEY, &admin);
        
        // Set the IPCM contract address
        env.storage().instance().set(&IPCM_CONTRACT_KEY, &ipcm_contract);
    }
    
    /// Mint a new NFT with a token ID and owner
    pub fn mint(env: Env, caller: Address, token_id: String, owner: Address, ipcm_key: String) {
        // Check if caller is admin
        Self::require_admin(&env, &caller);
        
        // Check if token already exists
        let mut tokens: Map<String, Address> = env.storage()
            .persistent()
            .get(&TOKENS_KEY)
            .unwrap_or_else(|| Map::new(&env));
            
        if tokens.contains_key(&token_id) {
            panic!("Token already exists");
        }
        
        // Assign the token to the owner
        tokens.set(token_id.clone(), owner.clone());
        env.storage().persistent().set(&TOKENS_KEY, &tokens);
        
        // Update the owner's tokens
        let owner_key = (OWNERS_KEY, owner.clone());
        let mut owner_tokens: Vec<String> = env.storage()
            .persistent()
            .get(&owner_key)
            .unwrap_or_else(|| Vec::new(&env));
            
        owner_tokens.push_back(token_id.clone());
        env.storage().persistent().set(&owner_key, &owner_tokens);
        
        // Store the IPCM key reference with the token
        let ipcm_ref_key = (IPCM_REF_KEY, token_id.clone());
        env.storage().persistent().set(&ipcm_ref_key, &ipcm_key);
        
        // Emit mint event
        env.events().publish(
            (MINT_EVENT, token_id.clone()),
            (token_id, owner, ipcm_key),
        );
    }
    
    /// Transfer an NFT from one owner to another
    pub fn transfer(env: Env, caller: Address, token_id: String, to: Address) {
        // Check if caller owns the token
        let tokens: Map<String, Address> = env.storage()
            .persistent()
            .get(&TOKENS_KEY)
            .unwrap_or_else(|| Map::new(&env));
            
        if !tokens.contains_key(&token_id) {
            panic!("Token does not exist");
        }
        
        let current_owner = tokens.get(token_id.clone()).unwrap();
        if current_owner != caller {
            panic!("Caller does not own this token");
        }
        
        caller.require_auth();
        
        // Remove token from current owner's list
        let owner_key = (OWNERS_KEY, current_owner.clone());
        let mut owner_tokens: Vec<String> = env.storage()
            .persistent()
            .get(&owner_key)
            .unwrap();
            
        let mut new_owner_tokens = Vec::new(&env);
        for i in 0..owner_tokens.len() {
            let t = owner_tokens.get(i).unwrap();
            if t != token_id {
                new_owner_tokens.push_back(t);
            }
        }
        
        env.storage().persistent().set(&owner_key, &new_owner_tokens);
        
        // Add token to new owner's list
        let new_owner_key = (OWNERS_KEY, to.clone());
        let mut new_owner_token_list: Vec<String> = env.storage()
            .persistent()
            .get(&new_owner_key)
            .unwrap_or_else(|| Vec::new(&env));
            
        new_owner_token_list.push_back(token_id.clone());
        env.storage().persistent().set(&new_owner_key, &new_owner_token_list);
        
        // Update token ownership mapping
        let mut updated_tokens = tokens.clone();
        updated_tokens.set(token_id.clone(), to.clone());
        env.storage().persistent().set(&TOKENS_KEY, &updated_tokens);
        
        // Emit transfer event
        env.events().publish(
            (TRANSFER_EVENT, token_id.clone()),
            (token_id, current_owner, to),
        );
    }
    
    /// Get the owner of a token
    pub fn owner_of(env: Env, token_id: String) -> Address {
        let tokens: Map<String, Address> = env.storage()
            .persistent()
            .get(&TOKENS_KEY)
            .unwrap_or_else(|| Map::new(&env));
            
        if !tokens.contains_key(&token_id) {
            panic!("Token does not exist");
        }
        
        tokens.get(token_id).unwrap()
    }
    
    /// Get the IPCM key for a token
    pub fn get_ipcm_key(env: Env, token_id: String) -> String {
        let ipcm_ref_key = (IPCM_REF_KEY, token_id.clone());
        
        if !env.storage().persistent().has(&ipcm_ref_key) {
            panic!("Token does not exist or has no IPCM key");
        }
        
        env.storage().persistent().get(&ipcm_ref_key).unwrap()
    }
    
    /// Get all tokens owned by an address
    pub fn tokens_of(env: Env, owner: Address) -> Vec<String> {
        let owner_key = (OWNERS_KEY, owner);
        
        env.storage()
            .persistent()
            .get(&owner_key)
            .unwrap_or_else(|| Vec::new(&env))
    }
    
    /// Burn (destroy) a token
    pub fn burn(env: Env, caller: Address, token_id: String) {
        // Check if caller owns the token
        let tokens: Map<String, Address> = env.storage()
            .persistent()
            .get(&TOKENS_KEY)
            .unwrap_or_else(|| Map::new(&env));
            
        if !tokens.contains_key(&token_id) {
            panic!("Token does not exist");
        }
        
        let current_owner = tokens.get(token_id.clone()).unwrap();
        if current_owner != caller {
            panic!("Caller does not own this token");
        }
        
        caller.require_auth();
        
        // Remove token from owner's list
        let owner_key = (OWNERS_KEY, current_owner.clone());
        let mut owner_tokens: Vec<String> = env.storage()
            .persistent()
            .get(&owner_key)
            .unwrap();
            
        let mut new_owner_tokens = Vec::new(&env);
        for i in 0..owner_tokens.len() {
            let t = owner_tokens.get(i).unwrap();
            if t != token_id {
                new_owner_tokens.push_back(t);
            }
        }
        
        env.storage().persistent().set(&owner_key, &new_owner_tokens);
        
        // Remove token from tokens mapping
        let mut updated_tokens = tokens.clone();
        updated_tokens.remove(&token_id);
        env.storage().persistent().set(&TOKENS_KEY, &updated_tokens);
        
        // Remove the IPCM key reference
        let ipcm_ref_key = (IPCM_REF_KEY, token_id.clone());
        env.storage().persistent().remove(&ipcm_ref_key);
        
        // Emit burn event
        env.events().publish(
            (BURN_EVENT, token_id.clone()),
            (token_id, current_owner),
        );
    }
    
    // Helper functions
    
    /// Check if the caller is the contract admin
    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        if *caller != admin {
            panic!("Caller is not the contract admin");
        }
        caller.require_auth();
    }
    
    /// Get the IPCM contract address
    fn get_ipcm_contract(env: &Env) -> Address {
        env.storage().instance().get(&IPCM_CONTRACT_KEY).unwrap()
    }
}

// Tests for the NFT contract
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};
    use soroban_sdk::{symbol_short, vec, Env};

    #[test]
    fn test_nft_contract() {
        let env = Env::default();
        
        // Deploy IPCM contract (mocked for this test)
        let ipcm_contract_address = Address::random(&env);
        
        // Deploy NFT contract
        let contract_id = env.register_contract(None, OctopusNFTContract);
        let admin = Address::random(&env);
        let user = Address::random(&env);
        
        let client = OctopusNFTContractClient::new(&env, &contract_id);
        
        // Initialize the contract
        client.initialize(&admin, &ipcm_contract_address);
        
        // Test minting a token
        let token_id = String::from_str(&env, "token123");
        let ipcm_key = String::from_str(&env, "ipcm_key_123");
        client.mint(&admin, &token_id, &user, &ipcm_key);
        
        // Verify owner
        let owner = client.owner_of(&token_id);
        assert_eq!(owner, user);
        
        // Verify IPCM key
        let retrieved_ipcm_key = client.get_ipcm_key(&token_id);
        assert_eq!(retrieved_ipcm_key, ipcm_key);
        
        // Verify user's tokens
        let user_tokens = client.tokens_of(&user);
        assert_eq!(user_tokens.len(), 1);
        assert_eq!(user_tokens.get(0).unwrap(), token_id);
        
        // Test token transfer
        let new_owner = Address::random(&env);
        client.transfer(&user, &token_id, &new_owner);
        
        // Verify new ownership
        let updated_owner = client.owner_of(&token_id);
        assert_eq!(updated_owner, new_owner);
        
        // Verify token lists updated correctly
        let user_tokens_after = client.tokens_of(&user);
        assert_eq!(user_tokens_after.len(), 0);
        
        let new_owner_tokens = client.tokens_of(&new_owner);
        assert_eq!(new_owner_tokens.len(), 1);
        assert_eq!(new_owner_tokens.get(0).unwrap(), token_id);
        
        // Test token burning
        client.burn(&new_owner, &token_id);
        
        // Verify token no longer exists
        let new_owner_tokens_after = client.tokens_of(&new_owner);
        assert_eq!(new_owner_tokens_after.len(), 0);
        
        // Attempting to get owner should fail
        let result = std::panic::catch_unwind(|| {
            client.owner_of(&token_id);
        });
        assert!(result.is_err());
    }
}
