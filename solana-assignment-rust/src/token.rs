use crate::constant::RPC_URL;
use crate::constant::DECIMALS;

use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::{
    program_pack::Pack,
    // signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use spl_token_interface::instruction as token_instruction;
use solana_system_interface::instruction as system_instruction;
// use spl_associated_token_account_interface::instruction as ata_instruction;
use std::io::{self, Write};
use mpl_token_metadata::ID as TOKEN_METADATA_PROGRAM_ID;
// use solana_sdk::pubkey::Pubkey;
use solana_address::Address;
use mpl_token_metadata::{
    instructions::CreateV1Builder,
    types::TokenStandard,
};
// use solana_sdk::instruction::{AccountMeta as SdkAccountMeta, Instruction as SdkInstruction};
use solana_client::rpc_config::CommitmentConfig;



pub fn create_and_transfer_spl_token() {
// let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(RPC_URL.to_string());
    println!("Using Solana RPC: {RPC_URL}");
    let keypair_path = dirs::home_dir()
    .expect("Could not find home directory")
    .join(".config/solana/id.json");
    match client.get_version() {
        Ok(version) => println!("Connected to devnet. Cluster version: {}", version.solana_core),
        Err(error) => eprintln!("Could not connect to devnet: {error}"),
    }

let payer = read_keypair_file(&keypair_path)
    .expect("Could not read Solana keypair");
let _payer_pubkey =solana_program::pubkey::Pubkey::new_from_array(payer.pubkey().to_bytes());

let mint_path = std::path::Path::new("mint-keypair.json");
let mint = read_keypair_file(mint_path)
    .expect("Could not read mint-keypair.json");
println!("Mint address: {}", mint.pubkey()); 
let mint_space = spl_token::state::Mint::LEN;
let rent = client
    .get_minimum_balance_for_rent_exemption(mint_space)
    .expect("Could not fetch mint rent");
    
let create_ata =
    spl_associated_token_account::instruction::create_associated_token_account_idempotent(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint.pubkey(),
        &spl_token::id(),
    );

// creating transaction locally
let recent_blockhash = match client.get_latest_blockhash() {
                Ok(blockhash) => blockhash,
                Err(error) => {
                    eprintln!("Could not fetch recent blockhash: {error}");
                    return;
                }
            };

let transaction = Transaction::new_signed_with_payer(
    &[create_ata],
    Some(&payer.pubkey()),
    &[&payer],
    recent_blockhash,
);
let owner_ata = spl_associated_token_account::get_associated_token_address(
    &payer.pubkey(),
    &mint.pubkey(),
);

let token_account = client
    .get_account(&owner_ata)
    .expect("Could not find your token account");

println!("ATA exists: {}", owner_ata);
println!("ATA account owner program: {}", token_account.owner);

// sending transaction to rpc [devnet,mainnet]
let signature = client
    .send_and_confirm_transaction(&transaction)
    .expect("ATA creation failed");

println!("Your token account: {owner_ata}");
println!("ATA creation transaction: {signature}");


println!("Created SPL token mint: {}", mint.pubkey());
println!("Transaction signature: {signature}");
println!("Wallet address: {}", payer.pubkey());
println!("Rent: {}", rent);
// println!("Mint Adress: {}", mint_address);
// println!("Wallet address: {}", mint_pubkey);
// println!("Wallet address: {}", payer.pubkey());
create_mint(&client,&payer,&mint);
// mint_tokens(&client, &payer, &mint, &create_ata, 100);
transfer_token(&client, &payer, &mint, &owner_ata);
create_token_metadata(&client, &payer, &mint);

    
}


fn transfer_token(client: &RpcClient,payer: &solana_sdk::signer::keypair::Keypair,mint: &Keypair,create_ata:&solana_address::Address,){
print!("Enter recipient's Solana devnet address: ");
io::stdout().flush().expect("Could not flush prompt");

let mut recipient_input = String::new();
io::stdin()
    .read_line(&mut recipient_input)
    .expect("Could not read address");

let recipient = recipient_input
    .trim()
    .parse::<solana_sdk::pubkey::Pubkey>()
    .expect("Invalid Solana address");
    

  let recipient_ata =
    spl_associated_token_account::get_associated_token_address(
        &recipient,
        &mint.pubkey(),
    );

let create_recipient_ata =
    spl_associated_token_account::instruction::create_associated_token_account_idempotent(
        &payer.pubkey(), // pays the creation cost if needed
        &recipient,      // owns the ATA
        &mint.pubkey(),
        &spl_token::id(),
    );  

let transfer = token_instruction::transfer_checked(
    &spl_token::id(),
    &create_ata,       // your source ATA
    &mint.pubkey(),   // mint verifies the decimals
    &recipient_ata,   // destination ATA
    &payer.pubkey(),  // source token-account owner signs
    &[],
    10_000_000,      // 10 tokens with 6 decimals
    DECIMALS,
)
.expect("Could not build transfer instruction");

let recent_blockhash = client
    .get_latest_blockhash()
    .expect("Could not fetch recent blockhash");

let transaction = Transaction::new_signed_with_payer(
    &[create_recipient_ata, transfer],
    Some(&payer.pubkey()), // optional freeze authority
    &[&payer],
    recent_blockhash,
);

let signature = client
    .send_and_confirm_transaction(&transaction)
    .expect("Token transfer transaction failed");

println!("Sent 10 tokens to {recipient}");
println!("Recipient ATA: {recipient_ata}");
println!("Transfer transaction: {signature}");


}


fn create_mint(client: &RpcClient,payer: &solana_sdk::signer::keypair::Keypair,mint: &Keypair) {
     
let mint_space = spl_token::state::Mint::LEN;
let rent = client
    .get_minimum_balance_for_rent_exemption(mint_space)
    .expect("Could not fetch mint rent");
    
let decimals = 6;
    
if client.get_account(&mint.pubkey()).is_ok() {
    println!("Mint already exists: {}", mint.pubkey());
} else {
    // Keep the existing mint creation transaction here.
    
    let create_mint_account = system_instruction::create_account(
    &payer.pubkey(),
    &mint.pubkey(),
    rent,
    mint_space as u64,
    &spl_token::id(),
);
let initialize_mint = token_instruction::initialize_mint(
    &spl_token::id(),
    &mint.pubkey(),

    &payer.pubkey(),
    Some(&payer.pubkey()),
    decimals,
)
.expect("Could not build initialize mint instruction");
let recent_blockhash = match client.get_latest_blockhash() {
                Ok(blockhash) => blockhash,
                Err(error) => {
                    eprintln!("Could not fetch recent blockhash: {error}");
                    return;
                }
            };

let transaction = Transaction::new_signed_with_payer(
    &[create_mint_account, initialize_mint],
    Some(&payer.pubkey()),
    &[&payer, &mint],
    recent_blockhash,
);

let _signature = client
    .send_and_confirm_transaction(&transaction)
    .expect("Mint creation transaction failed");


} }

pub fn create_token_metadata(
    client: &RpcClient,
    payer: &Keypair,
    mint: &Keypair,
) {
    let metadata_program = Address::from(TOKEN_METADATA_PROGRAM_ID.to_bytes());
    let mint_address = Address::from(mint.pubkey().to_bytes());

    let (metadata_pda, _bump) = Address::find_program_address(
        &[
            b"metadata",
            metadata_program.as_ref(),
            mint_address.as_ref(),
        ],
        &metadata_program,
    );

    println!("Metadata PDA: {metadata_pda}");

    let account_check = client.get_account_with_commitment(
        &metadata_pda,
        CommitmentConfig::confirmed(),
    );

    match account_check {
        Ok(response) if response.value.is_some() => {
            println!("Metadata already exists; skipping creation.");
        }
        Ok(_) => {
            let metadata_pubkey =
                solana_program::pubkey::Pubkey::new_from_array(metadata_pda.to_bytes());
            let mint_pubkey =
                solana_program::pubkey::Pubkey::new_from_array(mint.pubkey().to_bytes());
            let payer_pubkey =
                solana_program::pubkey::Pubkey::new_from_array(payer.pubkey().to_bytes());

            let metaplex_instruction = CreateV1Builder::new()
                .metadata(metadata_pubkey)
                .mint(mint_pubkey, true)
                .authority(payer_pubkey)
                .payer(payer_pubkey)
                .update_authority(payer_pubkey, true)
                .name("Tanjiro Token".to_string())
                .symbol("TKT".to_string())
                .uri(
                    "https://raw.githubusercontent.com/bakshiyogesh/spl-token-metadata/main/metadata.json"
                        .to_string(),
                )
                .seller_fee_basis_points(0)
                .primary_sale_happened(false)
                .is_mutable(true)
                .token_standard(TokenStandard::Fungible)
                .decimals(6)
                .instruction();

            let metadata_instruction = solana_sdk::instruction::Instruction {
                program_id: Address::from(metaplex_instruction.program_id.to_bytes()),
                accounts: metaplex_instruction
                    .accounts
                    .iter()
                    .map(|account| solana_sdk::instruction::AccountMeta {
                        pubkey: Address::from(account.pubkey.to_bytes()),
                        is_signer: account.is_signer,
                        is_writable: account.is_writable,
                    })
                    .collect(),
                data: metaplex_instruction.data,
            };

            let recent_blockhash = match client.get_latest_blockhash() {
                Ok(blockhash) => blockhash,
                Err(error) => {
                    eprintln!("Could not fetch recent blockhash: {error}");
                    return;
                }
            };

            let transaction = Transaction::new_signed_with_payer(
                &[metadata_instruction],
                Some(&payer.pubkey()),
                &[payer, mint],
                recent_blockhash,
            );

            match client.send_and_confirm_transaction(&transaction) {
                Ok(signature) => {
                    println!("Created token metadata.");
                    println!("Metadata transaction: {signature}");
                }
                Err(error) => eprintln!("Metadata creation failed: {error}"),
            }
        }
        Err(error) => {
            eprintln!("Could not check metadata account; skipping creation: {error}");
        }
    }
}