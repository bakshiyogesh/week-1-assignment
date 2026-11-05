// use solana_sdk::signer::keypair::Keypair;
// use std::{fs, path::PathBuf};

// fn main() {
//     let encoded = rpassword::prompt_password("Paste Phantom Solana private key:" )
//         .expect("Could not read private key");

//     let bytes = bs58::decode(encoded.trim())
//         .into_vec()
//         .expect("Private key is not valid base58");

//     let keypair = Keypair::try_from_base58_string(encoded.trim())
//     .expect("Invalid Solana base58 private key");
//    let bytes = keypair.to_bytes();

//     let output = dirs::home_dir()
//         .expect("Could not find home directory")
//         .join(".config/solana/phantom-owner.json");

//     fs::write(&output, serde_json::to_vec(&bytes).unwrap())
//         .expect("Could not write keypair file");

//     println!("Saved keypair for {} to {}", keypair.pubkey(), output.display());
// }
fn main(){

println!("hiiiii");}