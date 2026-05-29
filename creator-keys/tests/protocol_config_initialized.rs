//! Tests for the is_protocol_config_initialized read-only method.

use creator_keys::{CreatorKeysContract, CreatorKeysContractClient};
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_is_protocol_config_initialized_returns_false_before_initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    // Before any initialization, the flag should be false
    assert!(!client.is_protocol_config_initialized());
}

#[test]
fn test_is_protocol_config_initialized_returns_true_after_fee_config_is_set() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);

    client.set_fee_config(&admin, &9000u32, &1000u32);

    assert!(client.is_protocol_config_initialized());
}

#[test]
fn test_is_protocol_config_initialized_is_read_only() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);

    client.set_fee_config(&admin, &8000u32, &2000u32);

    let first_read = client.is_protocol_config_initialized();
    let second_read = client.is_protocol_config_initialized();

    assert_eq!(first_read, second_read);
    assert!(first_read);
}
