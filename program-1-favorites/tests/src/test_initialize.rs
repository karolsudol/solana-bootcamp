use std::str::FromStr;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::read_keypair_file,
        signer::Signer,
    },
    Client, Cluster,
};

#[test]
fn test_set_favorites() {
    // Setup the client
    let program_id = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS";
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(
        Cluster::Localnet,
        &payer,
        CommitmentConfig::confirmed()
    );
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    // Calculate PDA for favorites account
    let seeds = &[
        b"favorites",
        payer.pubkey().as_ref()
    ];
    let (favorites_pda, _bump) = Pubkey::find_program_address(seeds, &program_id);

    // Test data
    let favorite_number: u64 = 42;
    let favorite_color = "blue".to_string();
    let hobbies = vec!["reading".to_string(), "coding".to_string()];

    // Send transaction
    let tx = program
        .request()
        .accounts(program_1_favorites::accounts::SetFavorites {
            user: payer.pubkey(),
            favorites: favorites_pda,
            system_program: anchor_client::solana_sdk::system_program::ID,
        })
        .args(program_1_favorites::instruction::SetFavorites {
            number: favorite_number,
            color: favorite_color.clone(),
            hobbies: hobbies.clone(),
        })
        .send()
        .expect("Failed to send transaction");

    println!("Transaction signature: {}", tx);

    let favorites_account = program
        .account::<program_1_favorites::Favorites>(favorites_pda)
        .expect("Failed to fetch favorites account");

    assert_eq!(favorites_account.number, favorite_number);
    assert_eq!(favorites_account.color, favorite_color);
    assert_eq!(favorites_account.hobbies, hobbies);
}
