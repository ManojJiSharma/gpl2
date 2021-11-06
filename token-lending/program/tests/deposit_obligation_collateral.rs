#![cfg(feature = "test-bpf")]

mod helpers;

use helpers::*;
use gemachain_program_test::*;
use gemachain_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use gpl_token::instruction::approve;
use gpl_token_lending::{
    instruction::deposit_obligation_collateral, processor::process_instruction,
    state::INITIAL_COLLATERAL_RATIO,
};

#[tokio::test]
async fn test_success() {
    let mut test = ProgramTest::new(
        "gpl_token_lending",
        gpl_token_lending::id(),
        processor!(process_instruction),
    );

    // limit to track compute unit increase
    test.set_bpf_compute_max_units(38_000);

    const GEMA_DEPOSIT_AMOUNT_CARATS: u64 = 10 * CARATS_TO_GEMA * INITIAL_COLLATERAL_RATIO;
    const GEMA_RESERVE_COLLATERAL_CARATS: u64 = 2 * GEMA_DEPOSIT_AMOUNT_CARATS;

    let user_accounts_owner = Keypair::new();
    let user_transfer_authority = Keypair::new();

    let lending_market = add_lending_market(&mut test);

    let gema_oracle = add_gema_oracle(&mut test);
    let gema_test_reserve = add_reserve(
        &mut test,
        &lending_market,
        &gema_oracle,
        &user_accounts_owner,
        AddReserveArgs {
            user_liquidity_amount: GEMA_RESERVE_COLLATERAL_CARATS,
            liquidity_amount: GEMA_RESERVE_COLLATERAL_CARATS,
            liquidity_mint_decimals: 9,
            liquidity_mint_pubkey: gpl_token::native_mint::id(),
            config: TEST_RESERVE_CONFIG,
            mark_fresh: true,
            ..AddReserveArgs::default()
        },
    );

    let test_obligation = add_obligation(
        &mut test,
        &lending_market,
        &user_accounts_owner,
        AddObligationArgs::default(),
    );

    let (mut banks_client, payer, recent_blockhash) = test.start().await;

    test_obligation.validate_state(&mut banks_client).await;

    let initial_collateral_supply_balance =
        get_token_balance(&mut banks_client, gema_test_reserve.collateral_supply_pubkey).await;
    let initial_user_collateral_balance =
        get_token_balance(&mut banks_client, gema_test_reserve.user_collateral_pubkey).await;

    let mut transaction = Transaction::new_with_payer(
        &[
            approve(
                &gpl_token::id(),
                &gema_test_reserve.user_collateral_pubkey,
                &user_transfer_authority.pubkey(),
                &user_accounts_owner.pubkey(),
                &[],
                GEMA_DEPOSIT_AMOUNT_CARATS,
            )
            .unwrap(),
            deposit_obligation_collateral(
                gpl_token_lending::id(),
                GEMA_DEPOSIT_AMOUNT_CARATS,
                gema_test_reserve.user_collateral_pubkey,
                gema_test_reserve.collateral_supply_pubkey,
                gema_test_reserve.pubkey,
                test_obligation.pubkey,
                lending_market.pubkey,
                test_obligation.owner,
                user_transfer_authority.pubkey(),
            ),
        ],
        Some(&payer.pubkey()),
    );

    transaction.sign(
        &vec![&payer, &user_accounts_owner, &user_transfer_authority],
        recent_blockhash,
    );
    assert!(banks_client.process_transaction(transaction).await.is_ok());

    // check that collateral tokens were transferred
    let collateral_supply_balance =
        get_token_balance(&mut banks_client, gema_test_reserve.collateral_supply_pubkey).await;
    assert_eq!(
        collateral_supply_balance,
        initial_collateral_supply_balance + GEMA_DEPOSIT_AMOUNT_CARATS
    );
    let user_collateral_balance =
        get_token_balance(&mut banks_client, gema_test_reserve.user_collateral_pubkey).await;
    assert_eq!(
        user_collateral_balance,
        initial_user_collateral_balance - GEMA_DEPOSIT_AMOUNT_CARATS
    );
}
