//! Regression tests for unpause restoring full buy and sell functionality (#468).
//!
//! Covers: buy reverts while paused, buy succeeds immediately after unpause,
//! sell succeeds immediately after unpause, and post-unpause state matches
//! expected values as if the pause never occurred.

mod contract_test_env;

use contract_test_env::{
    register_creator_keys, register_test_creator, set_key_price_for_tests, test_env_with_auths,
};
use creator_keys::ContractError;
use soroban_sdk::{testutils::Address as _, Address};

const KEY_PRICE: i128 = 1_000;

fn setup_with_admin(
    env: &soroban_sdk::Env,
) -> (
    creator_keys::CreatorKeysContractClient<'_>,
    Address,
    Address,
) {
    let (client, _) = register_creator_keys(env);
    set_key_price_for_tests(env, &client, KEY_PRICE);
    let admin = Address::generate(env);
    client.set_protocol_admin(&admin, &admin);
    let creator = register_test_creator(env, &client, "alice");
    (client, admin, creator)
}
