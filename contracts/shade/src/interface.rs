use soroban_sdk::{contracttrait, Address, BytesN, Env};

#[contracttrait]
pub trait ShadeTrait {
    fn initialize(env: Env, admin: Address);
    fn get_admin(env: Env) -> Address;
    fn upgrade(env: Env, admin: Address, new_wasm_hash: BytesN<32>);
    fn add_accepted_token(env: Env, admin: Address, token: Address);
    fn remove_accepted_token(env: Env, admin: Address, token: Address);
    fn is_accepted_token(env: Env, token: Address) -> bool;
}
