use {
    anchor_lang::{
        prelude::{Pubkey, rent},
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_amm_lifecycle() {
    let program_id = amm::id();
    let payer = Keypair::new();
    let treasury = Keypair::new();

    let mint_a = Pubkey::new_unique();
    let mint_b = Pubkey::new_unique();
    let lp_mint = Pubkey::new_unique();

    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/amm.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&treasury.pubkey(), 1_000_000_000).unwrap();

    let pool_key = Pubkey::find_program_address(
        &[amm::constants::POOL_SEED, mint_a.as_ref(), mint_b.as_ref()],
        &program_id,
    ).0;

    let config_key = Pubkey::find_program_address(
        &[amm::constants::CONFIG_SEED],
        &program_id,
    ).0;

    // 1. Initialize Pool
    let instruction = Instruction::new_with_bytes(
        program_id,
        &amm::instruction::InitializePool {
            initial_reserves_a: 1000,
            initial_reserves_b: 1000,
            amplification_coefficient: 100_000_000,
        }.data(),
        amm::accounts::InitializePool {
            config: config_key,
            pool: pool_key,
            mint_a,
            mint_b,
            lp_mint,
            treasury: treasury.pubkey(),
            user: payer.pubkey(),
            system_program: system_program::ID,
            rent: rent::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();
    let res = svm.send_transaction(tx);
    if let Err(ref e) = res {
        println!("Transaction failed: {:?}", e);
    }
    assert!(res.is_ok());

    // Verify Pool State
    let pool_account = svm.get_account(&pool_key).unwrap();
    let mut data: &[u8] = &pool_account.data;
    let pool_state = amm::state::Pool::try_deserialize(&mut data).unwrap();
    assert_eq!(pool_state.reserve_a, 1000);
    assert_eq!(pool_state.reserve_b, 1000);
}
