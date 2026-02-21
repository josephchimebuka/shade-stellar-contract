use crate::components::{admin as admin_component, core as core_component};
use crate::errors::ContractError;
use crate::events;
use crate::interface::ShadeTrait;
use crate::types::{ContractInfo, DataKey};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, BytesN, Env};

#[contract]
pub struct Shade;

#[contractimpl]
impl ShadeTrait for Shade {
    fn initialize(env: Env, admin: Address) {
        if env.storage().persistent().has(&DataKey::Admin) {
            panic_with_error!(&env, ContractError::AlreadyInitialized);
        }
        let contract_info = ContractInfo {
            admin: admin.clone(),
            timestamp: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage()
            .persistent()
            .set(&DataKey::ContractInfo, &contract_info);
        events::publish_initialized_event(&env, admin, env.ledger().timestamp());
    }
    fn get_admin(env: Env) -> Address {
        core_component::get_admin(&env)
    }

    fn upgrade(env: Env, admin: Address, new_wasm_hash: BytesN<32>) {
        core_component::assert_admin(&env, &admin);
        env.deployer()
            .update_current_contract_wasm(new_wasm_hash.clone());
        events::publish_contract_upgraded_event(&env, new_wasm_hash, env.ledger().timestamp());
    }

    fn add_accepted_token(env: Env, admin: Address, token: Address) {
        admin_component::add_accepted_token(&env, &admin, &token);
    }

    fn remove_accepted_token(env: Env, admin: Address, token: Address) {
        admin_component::remove_accepted_token(&env, &admin, &token);
    }

    fn is_accepted_token(env: Env, token: Address) -> bool {
        admin_component::is_accepted_token(&env, &token)
    }
}
