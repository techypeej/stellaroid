#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

    // Test 1: Happy Path - Successful Registration
    #[test]
    fn test_successful_registration() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CashtellarContract);
        let client = CashtellarContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let receipt = Symbol::new(&env, "tx_12345");

        env.mock_all_auths();
        let result = client.register_receipt(&user, &receipt);
        assert_eq!(result, Symbol::new(&env, "SUCCESS"));
    }

    // Test 2: Edge Case - Duplicate Detection
    #[test]
    #[should_panic(expected = "Receipt already validated or duplicate!")]
    fn test_duplicate_registration_fails() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CashtellarContract);
        let client = CashtellarContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let receipt = Symbol::new(&env, "tx_12345");

        env.mock_all_auths();
        client.register_receipt(&user, &receipt);
        client.register_receipt(&user, &receipt); // Should panic here
    }

    // Test 3: State Verification - Storage check
    #[test]
    fn test_state_verification() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CashtellarContract);
        let client = CashtellarContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let receipt = Symbol::new(&env, "tx_999");

        env.mock_all_auths();
        client.register_receipt(&user, &receipt);

        let stored_owner = client.verify_receipt(&receipt);
        assert_eq!(stored_owner, user);
    }
}