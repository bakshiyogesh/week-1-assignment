use mpl_core::instructions::CreateV1Builder;
use mpl_core::instructions::BurnV1Builder;
use solana_sdk::signer::keypair::{read_keypair_file, Keypair};
use solana_sdk::signer::Signer;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    transaction::Transaction,
    // program_pack::Pack,
    // // signature::Keypair,
    // signer::Signer,
};
use mpl_core::instructions::UpdateV1Builder;
use mpl_core::instructions::TransferV1Builder;
use std::io::{self, Write};

pub fn create_core_nft() {
let client = RpcClient::new(crate::constant::RPC_URL.to_string());

let payer_path = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config/solana/id.json");

let payer = read_keypair_file(payer_path)
        .expect("Could not read Solana keypair");
let asset = read_keypair_file("new-nft-keypair.json")
        .expect("Could not read Core NFT keypair");

let create_asset_instruction = CreateV1Builder::new()
    .asset(asset.pubkey())
    .payer(payer.pubkey())
    .owner(Some(payer.pubkey()))
    .update_authority(Some(payer.pubkey()))
    .name("Tanjiro Core NFT".to_string())
    .uri(
        "https://raw.githubusercontent.com/bakshiyogesh/spl-token-metadata/main/new-nft-metadat.json"
            .to_string(),
    )
    .instruction();if client.get_account(&asset.pubkey()).is_ok() {
    println!("Core asset already exists: {}", asset.pubkey());
    return;
}
    
if client.get_account(&asset.pubkey()).is_ok() {
    println!("Core asset already exists: {}", asset.pubkey());
    return;
}    

let recent_blockhash = client
    .get_latest_blockhash()
    .expect("Could not fetch recent blockhash");

let transaction = Transaction::new_signed_with_payer(
    &[create_asset_instruction],
    Some(&payer.pubkey()),
    &[&payer, &asset],
    recent_blockhash,
);

let signature = client
    .send_and_confirm_transaction(&transaction)
    .expect("Core NFT creation failed");

println!("Created Core NFT asset: {}", asset.pubkey());
println!("Creation transaction: {signature}");        
      
}

pub fn update_core_nft() {
    let client = RpcClient::new(crate::constant::RPC_URL.to_string());

    let payer_path = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config/solana/id.json");
    let payer = read_keypair_file(payer_path)
        .expect("Could not read wallet keypair");

    let asset = read_keypair_file("new-nft-keypair.json")
        .expect("Could not read Core NFT keypair");

    let update_instruction = UpdateV1Builder::new()
        .asset(asset.pubkey())
        .payer(payer.pubkey())
        .authority(Some(payer.pubkey()))
        .new_name("Tanjiro Core NFT Updated".to_string())
      .new_uri("https://raw.githubusercontent.com/bakshiyogesh/spl-token-metadata/main/new-nft-metadat.json".to_string())
        .instruction();

    let recent_blockhash = client
    .get_latest_blockhash()
    .expect("Could not fetch recent blockhash");

let transaction = Transaction::new_signed_with_payer(
    &[update_instruction],
    Some(&payer.pubkey()),
    &[&payer],
    recent_blockhash,
);

let signature = client
    .send_and_confirm_transaction(&transaction)
    .expect("Core NFT update failed");

println!("Updated Core NFT: {}", asset.pubkey());
println!("Update transaction: {signature}");
}

pub fn transfer_core_nft() {
    let client = RpcClient::new(crate::constant::RPC_URL.to_string());

    let payer_path = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config/solana/id.json");
    let payer = read_keypair_file(payer_path)
        .expect("Could not read wallet keypair");

    let asset = read_keypair_file("new-nft-keypair.json")
        .expect("Could not read Core NFT keypair");

    print!("Enter recipient's Solana devnet address: ");
    io::stdout().flush().expect("Could not flush prompt");

    let mut recipient_input = String::new();
    io::stdin()
        .read_line(&mut recipient_input)
        .expect("Could not read address");

    let reciepient = recipient_input
        .trim()
        .parse::<solana_address::Address>()
        .expect("Invalid Solana address");

    let transfer_instruction = TransferV1Builder::new()
        .asset(asset.pubkey())
        .payer(payer.pubkey())
        .authority(Some(payer.pubkey()))
        .new_owner(reciepient)
        .instruction();


let recent_blockhash=client.get_latest_blockhash().expect("Couldn't fetch recent blockhash");

let transaction=Transaction::new_signed_with_payer(&[transfer_instruction],Some(&payer.pubkey()),&[&payer],recent_blockhash);

let signature=client.send_and_confirm_transaction(&transaction).expect("Core NFT Transfer failed");

println!("Transfered Core NFT {} to {reciepient}", asset.pubkey());
println!("Transfer transaction:{signature}");

    // Build and send the transaction here.
}

pub fn burn_core_nft() {
    let client = RpcClient::new(crate::constant::RPC_URL.to_string());

    let payer_path = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config/solana/id.json");
    let payer = read_keypair_file(payer_path)
        .expect("Could not read fee-payer keypair");

    let owner = read_keypair_file(
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config/solana/id.json"),
    )
    .expect("Could not read current-owner keypair");

    let asset = read_keypair_file("new-nft-keypair.json")
        .expect("Could not read New NFT keypair");

    println!("Asset: {}", asset.pubkey());
    println!("Current owner signer: {}", owner.pubkey());
    println!("Rent recipient / fee payer: {}", payer.pubkey());

    let burn_instruction = BurnV1Builder::new()
        .asset(asset.pubkey())
        .payer(payer.pubkey())
        .authority(Some(owner.pubkey()))
        .instruction();

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Could not fetch recent blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[burn_instruction],
        Some(&payer.pubkey()),
        &[&payer, &owner],
        recent_blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Core NFT burn failed");

    println!("Core NFT burned: {}", asset.pubkey());
    println!("Burn transaction: {signature}");
}