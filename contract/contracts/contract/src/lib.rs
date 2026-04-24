#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Map, String, Symbol, Vec,
};

// ─── Data Types ────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug)]
pub struct TreeCertificate {
    pub token_id: u64,
    pub owner: Address,
    pub planter: Address,
    pub species: String,
    pub location: String,      // e.g. "lat:28.6139,lon:77.2090"
    pub planted_at: u64,       // Unix timestamp
    pub verified_by: Address,
    pub verified_at: u64,
    pub metadata_uri: String,  // IPFS URI for off-chain photo/data
}

#[contracttype]
pub enum DataKey {
    Certificate(u64),          // token_id → TreeCertificate
    OwnerTokens(Address),      // owner → Vec<u64>
    TotalSupply,
    Admin,
    Verifiers,
}

// ─── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct TreeNFTContract;

#[contractimpl]
impl TreeNFTContract {

    // ── Initialisation ────────────────────────────────────────────────────────

    /// Deploy the contract with an admin address.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &0u64);
        env.storage()
            .instance()
            .set(&DataKey::Verifiers, &Vec::<Address>::new(&env));
    }

    // ── Admin helpers ─────────────────────────────────────────────────────────

    fn get_admin(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    fn require_admin(env: &Env) {
        Self::get_admin(env).require_auth();
    }

    fn require_verifier(env: &Env, caller: &Address) {
        let verifiers: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Verifiers)
            .unwrap_or(Vec::new(env));
        if !verifiers.contains(caller) && *caller != Self::get_admin(env) {
            panic!("caller is not an authorized verifier");
        }
        caller.require_auth();
    }

    // ── Verifier management ───────────────────────────────────────────────────

    /// Add a trusted verifier (admin only).
    pub fn add_verifier(env: Env, verifier: Address) {
        Self::require_admin(&env);
        let mut verifiers: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Verifiers)
            .unwrap_or(Vec::new(&env));
        if !verifiers.contains(&verifier) {
            verifiers.push_back(verifier);
            env.storage()
                .instance()
                .set(&DataKey::Verifiers, &verifiers);
        }
    }

    /// Remove a verifier (admin only).
    pub fn remove_verifier(env: Env, verifier: Address) {
        Self::require_admin(&env);
        let verifiers: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Verifiers)
            .unwrap_or(Vec::new(&env));
        let mut new_verifiers = Vec::new(&env);
        for v in verifiers.iter() {
            if v != verifier {
                new_verifiers.push_back(v);
            }
        }
        env.storage()
            .instance()
            .set(&DataKey::Verifiers, &new_verifiers);
    }

    // ── Core NFT actions ──────────────────────────────────────────────────────

    /// Mint a new tree certificate (verifier only).
    /// Returns the newly assigned token_id.
    pub fn mint_certificate(
        env: Env,
        verifier: Address,
        planter: Address,
        recipient: Address,    // owner of the NFT
        species: String,
        location: String,
        planted_at: u64,
        metadata_uri: String,
    ) -> u64 {
        Self::require_verifier(&env, &verifier);

        let token_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0u64)
            + 1;

        let cert = TreeCertificate {
            token_id,
            owner: recipient.clone(),
            planter,
            species,
            location,
            planted_at,
            verified_by: verifier,
            verified_at: env.ledger().timestamp(),
            metadata_uri,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Certificate(token_id), &cert);

        // Update owner index
        let mut tokens: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerTokens(recipient.clone()))
            .unwrap_or(Vec::new(&env));
        tokens.push_back(token_id);
        env.storage()
            .persistent()
            .set(&DataKey::OwnerTokens(recipient), &tokens);

        // Increment supply
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &token_id);

        // Emit event
        env.events().publish(
            (symbol_short!("mint"), symbol_short!("tree")),
            token_id,
        );

        token_id
    }

    /// Transfer a certificate to a new owner.
    pub fn transfer(env: Env, from: Address, to: Address, token_id: u64) {
        from.require_auth();

        let mut cert: TreeCertificate = env
            .storage()
            .persistent()
            .get(&DataKey::Certificate(token_id))
            .expect("token not found");

        if cert.owner != from {
            panic!("caller does not own this token");
        }

        // Remove from sender's list
        let tokens: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerTokens(from.clone()))
            .unwrap_or(Vec::new(&env));
        let mut new_tokens = Vec::new(&env);
        for t in tokens.iter() {
            if t != token_id {
                new_tokens.push_back(t);
            }
        }
        env.storage()
            .persistent()
            .set(&DataKey::OwnerTokens(from), &new_tokens);

        // Add to recipient's list
        let mut to_tokens: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerTokens(to.clone()))
            .unwrap_or(Vec::new(&env));
        to_tokens.push_back(token_id);
        env.storage()
            .persistent()
            .set(&DataKey::OwnerTokens(to.clone()), &to_tokens);

        // Update cert owner
        cert.owner = to;
        env.storage()
            .persistent()
            .set(&DataKey::Certificate(token_id), &cert);

        env.events().publish(
            (symbol_short!("transfer"), symbol_short!("tree")),
            token_id,
        );
    }

    // ── Queries ───────────────────────────────────────────────────────────────

    /// Fetch a certificate by token_id.
    pub fn get_certificate(env: Env, token_id: u64) -> TreeCertificate {
        env.storage()
            .persistent()
            .get(&DataKey::Certificate(token_id))
            .expect("token not found")
    }

    /// List all token_ids owned by an address.
    pub fn tokens_of(env: Env, owner: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::OwnerTokens(owner))
            .unwrap_or(Vec::new(&env))
    }

    /// Total certificates ever minted.
    pub fn total_supply(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0u64)
    }

    /// Current admin address.
    pub fn admin(env: Env) -> Address {
        Self::get_admin(&env)
    }

    /// All authorised verifiers.
    pub fn verifiers(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Verifiers)
            .unwrap_or(Vec::new(&env))
    }
}