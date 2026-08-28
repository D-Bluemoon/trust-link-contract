#![cfg(test)]
use super::*;
use soroban_sdk::{Env, Address, testutils::Address as _};

#[test]
fn test_auto_release_triggers_past_deadline() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EscrowContract);
    let client = EscrowContractClient::new(&env, &contract_id);

    let order_id = 101;
    let amount = 5000;
    let deadline = 2000;

    env.ledger().set_timestamp(1000);
    client.initialize_order(&order_id, &amount, &deadline);

    // Attempting auto-release early must return false safely
    assert_eq!(client.auto_release(&order_id), false);

    // Fast-forward the network clock past the threshold
    env.ledger().set_timestamp(3000);

    // Auto-release executes successfully, passing acceptance criteria
    assert_eq!(client.auto_release(&order_id), true);
}
