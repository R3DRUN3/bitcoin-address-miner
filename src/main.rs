use bitcoin::{Address, PublicKey, Network};
use bitcoin::secp256k1::{rand, Secp256k1, PublicKey as SecpPublicKey};
use hex::encode_upper;
use std::time::Instant;

fn check_allowed_chars(starting_letters: &str) -> bool {
    let allowed_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    starting_letters.chars().all(|c| allowed_chars.contains(c))
}

async fn mine_address(starting_letters: String) {
    if !check_allowed_chars(&starting_letters) {
        println!("Allowed chars: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
        std::process::exit(1);
    }

    let secp = Secp256k1::new();

    // Initialize an Instant to measure elapsed time.
    let start_time = Instant::now();

    loop {
        // Generate random key pair.
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());

        // Convert secp256k1::PublicKey to bitcoin::PublicKey.
        let bitcoin_public_key =
            PublicKey::from(SecpPublicKey::from_slice(&public_key.serialize()[..]).unwrap());

        // Generate pay-to-pubkey-hash address.
        let address = Address::p2pkh(&bitcoin_public_key, Network::Bitcoin);

        // Ignore the specified number of characters when checking for a match.
        let address_tail = &address.to_string()[1..]; // Start from the second character onward

        // Uncomment the following line for debug: print the current address being attempted.
        //println!("Trying address: {}", address);

        // Check if the address (excluding the specified characters) starts with the desired letters.
        if address_tail.starts_with(&starting_letters) {
            println!("Found matching address: {}", address);
            println!("Private Key: {}", encode_upper(secret_key.as_ref()));
            println!("Public Key: {}", bitcoin_public_key);
            println!("Elapsed Time: {:.2} minutes", start_time.elapsed().as_secs_f64() / 60.0);
            std::process::exit(0);
        }
    }
}

#[tokio::main]
async fn main() {
    // Ask the user for the starting letters.
    println!("[BITCOIN KEY/ADDRESS MINING] \n Enter the starting letters for the address: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let starting_letters = input.trim().to_string();
    println!("\n Starting mining...this may take many minutes (or even days) to complete");

    // Check if all characters in starting_letters are allowed.
    if !check_allowed_chars(&starting_letters) {
        println!("You entered a wrong char - allowed chars: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
        std::process::exit(1);
    }

    // Run multiple async threads to mine addresses concurrently.
    let mut handles = vec![];
    // 50 parallel threads
    for _ in 0..50 {
        let handle = tokio::spawn(mine_address(starting_letters.clone()));
        handles.push(handle);
    }

    // Wait for all threads to complete.
    for handle in handles {
        handle.await.expect("Error in thread");
    }
}
