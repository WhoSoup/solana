use solana_client::rpc_client::RpcClient;
use solana_core::test_validator::TestValidator;
use solana_tokens::commands::test_process_distribute_tokens_with_client;

#[test]
fn test_process_distribute_with_rpc_client() {
    solana_logger::setup();

    let test_validator = TestValidator::with_no_fees();

    let client = RpcClient::new(test_validator.rpc_url());
    test_process_distribute_tokens_with_client(&client, test_validator.mint_keypair(), None);

    test_validator.close();
}
