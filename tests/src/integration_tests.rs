#[cfg(test)]
mod tests {
    use casper_engine_test_support::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
        DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_GENESIS_CONFIG,
        DEFAULT_GENESIS_CONFIG_HASH, DEFAULT_PAYMENT, DEFAULT_RUN_GENESIS_REQUEST,
        MINIMUM_ACCOUNT_CREATION_BALANCE,
    };
    use casper_execution_engine::core::engine_state::{
        run_genesis_request::RunGenesisRequest, GenesisAccount,
    };
    use casper_types::{
        account::AccountHash, runtime_args, system::mint, ContractHash, Key, Motes, PublicKey,
        RuntimeArgs, SecretKey, U256, U512,
    };
    use once_cell::sync::Lazy;
    use std::path::PathBuf;
    // from
    const EXAMPLE_ERC20_TOKEN: &str = "contract.wasm";

    static ACCOUNT_1_SECRET_KEY: Lazy<SecretKey> =
        Lazy::new(|| SecretKey::ed25519_from_bytes(&[221u8; 32]).unwrap());
    static ACCOUNT_1_PUBLIC_KEY: Lazy<PublicKey> =
        Lazy::new(|| PublicKey::from(&*ACCOUNT_1_SECRET_KEY));
    static ACCOUNT_1_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_1_PUBLIC_KEY.to_account_hash());
    const ERC20_TOKEN_CONTRACT_KEY: &str = "erc20_token_contract";
    const ARG_NAME: &str = "name";
    const ARG_SYMBOL: &str = "symbol";
    const ARG_DECIMALS: &str = "decimals";
    const ARG_TOTAL_SUPPLY: &str = "total_supply";

    const TOKEN_NAME: &str = "CasperTest";
    const TOKEN_SYMBOL: &str = "CSPRT";
    const TOKEN_DECIMALS: u8 = 100;
    const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000;
    // to
    #[derive(Copy, Clone)]
    struct TestContext {
        erc20_token: ContractHash,
    }
    fn setup() -> (InMemoryWasmTestBuilder, TestContext) {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

        let id: Option<u64> = None;
        let transfer_1_args = runtime_args! {
            mint::ARG_TARGET => *ACCOUNT_1_ADDR,
            mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
            mint::ARG_ID => id,
        };
        let transfer_request_1 =
            ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_1_args).build();
        let install_request_1 = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            EXAMPLE_ERC20_TOKEN,
            runtime_args! {
                ARG_NAME => TOKEN_NAME,
                ARG_SYMBOL => TOKEN_SYMBOL,
                ARG_DECIMALS => TOKEN_DECIMALS,
                ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
            },
        )
        .build();
        builder.exec(transfer_request_1).expect_success().commit();
        builder.exec(install_request_1).expect_success().commit();
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("should have account");
        let erc20_token = account
            .named_keys()
            .get(ERC20_TOKEN_CONTRACT_KEY)
            .and_then(|key| key.into_hash())
            .map(ContractHash::new)
            .expect("should have contract hash");
        let test_context = TestContext { erc20_token };
        (builder, test_context)
    }
    #[test]
    fn should_have_queryable_properties() {
        let (mut builder, TestContext { erc20_token, .. }) = setup();

        let name: String = builder.get_value(erc20_token, ARG_NAME);
        assert_eq!(name, TOKEN_NAME);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
