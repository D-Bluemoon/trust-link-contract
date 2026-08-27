#![cfg(test)]
use super::*;
use soroban_sdk::{Env, Address, testutils::Address as _};

/// Helper setup struct to initialize contract state for isolation tests
struct TestFixture {
    env: Env,
    contract_id: Address,
    client: TrustLinkContractClient<'static>,
}

impl TestFixture {
    fn setup(env: Env) -> Self {
        let contract_id = env.register_contract(None, TrustLinkContract);
        let client = TrustLinkContractClient::new(&env, &contract_id);
        TestFixture { env, contract_id, client }
    }
}

#[test]
fn test_default_arbitration_fee_matches_baseline() {
    let env = Env::default();
    let fixture = TestFixture::setup(env);

    // Requirement: Ensure backward compatibility with original baseline configurations
    let expected_basis_points = 100; // 1% default fee baseline
    let current_fee = fixture.client.get_arbitration_fee_bp();

    assert_eq!(current_fee, expected_basis_points);
}

#[test]
fn test_calculate_fee_amount_valid_scale() {
    let env = Env::default();
    let fixture = TestFixture::setup(env);

    // 10,000 native tokens with a 100 basis point fee (1%) should calculate to 100 tokens
    let dispute_amount: i128 = 10_000;
    let expected_fee_payout: i128 = 100;

    let calculated_fee = fixture.client.calculate_arbitration_fee(&dispute_amount);
    assert_eq!(calculated_fee, expected_fee_payout);
}

#[test]
fn test_arbitration_fee_upper_boundary_limits() {
    let env = Env::default();
    let fixture = TestFixture::setup(env);
    let admin = Address::generate(&env);

    // Acceptance Criteria: Verify logic operates correctly near maximum fee boundaries (e.g., 500 bp / 5%)
    let custom_high_fee = 500;
    fixture.client.set_arbitration_fee_bp(&admin, &custom_high_fee);

    let dispute_amount: i128 = 1_000;
    let expected_fee: i128 = 50;

    assert_eq!(fixture.client.calculate_arbitration_fee(&dispute_amount), expected_fee);
}

#[test]
#[should_panic(expected = "Fee exceeds maximum allowed basis points")]
fn test_arbitration_fee_exceeds_max_boundary_panics() {
    let env = Env::default();
    let fixture = TestFixture::setup(env);
    let admin = Address::generate(&env);

    // Testing Notes: Validate protection edge cases against malicious configurations
    let invalid_high_fee = 10_001;
    fixture.client.set_arbitration_fee_bp(&admin, &invalid_high_fee);
}
