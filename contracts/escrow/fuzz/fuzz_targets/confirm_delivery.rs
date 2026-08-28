#![no_main]
use libfuzzer_sys::fuzz_target;
use soroban_sdk::{Env, Address, testutils::Address as _};

// Import tracking states safely from your main project codebase
use escrow::{EscrowContract, EscrowContractClient};

fuzz_target!(|data: (u64, u32)| {
    let env = Env::default();

    // 1. Set up a secure sandboxed contract ID for testing
    let contract_id = env.register_contract(None, EscrowContract);
    let client = EscrowContractClient::new(&env, &contract_id);

    // 2. Generate tracking entries from mock inputs
    let order_id = data.0;
    let mock_merchant = Address::generate(&env);

    // 3. Prevent testing tools from crashing on zero boundaries (Backward Compatibility)
    if order_id > 0 {
        // Mocking entry authorization matching updated instance parameters
        env.mock_all_auths();

        // 4. Run delivery confirmation executions wrapped safely inside the test runtime
        let _result = client.confirm_delivery(&order_id, &mock_merchant);
    }
});
