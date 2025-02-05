use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
        system_program,
    },
    Client, Cluster,
};

#[test]
fn test_initialize() {
    let program_id = "FEwiJUyqCWJJyp2ucM9XzcHq5BD6iYjvMUN6TpiwVH1J";
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    // Generate a new keypair for the counter account
    let counter = anchor_client::solana_sdk::signature::Keypair::new();

    let tx = program
        .request()
        .accounts(program_0_anchor_basics::accounts::Initialize {
            payer: payer.pubkey(),
            counter: counter.pubkey(),
            system_program: system_program::ID,
        })
        .args(program_0_anchor_basics::instruction::Initialize {})
        .signer(&counter)
        .send()
        .expect("Failed to initialize counter");

    println!("Your transaction signature {}", tx);
}