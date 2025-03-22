use solana_client::rpc_client::RpcClient;
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;
use rand::Rng;
use tokio;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let recipient_address = env::var("RECIPIENT_ADDRESS").expect("RECIPIENT_ADDRESS must be set");
    let private_key_bytes_str = env::var("PRIVATE_KEY_BYTES").expect("PRIVATE_KEY_BYTES must be set");
    let num_tasks_str = env::var("NUM_TASKS").unwrap_or_else(|_| "128".to_string()); // Default to 128 if not set

    let client = Arc::new(RpcClient::new(&rpc_url));
    let private_key_bytes: Vec<u8> = private_key_bytes_str
        .split(',')
        .map(|s| s.trim().parse::<u8>().expect("Invalid byte in PRIVATE_KEY_BYTES")) // Corrected line
        .collect();

    let sender_keypair = Keypair::from_bytes(&private_key_bytes)?;
    let recipient_pubkey = Pubkey::from_str(&recipient_address)?;
    let num_tasks: usize = num_tasks_str.parse().expect("Invalid NUM_TASKS");

    let block_height = client.get_block_height()?;
    println!("Current block height: {}", block_height);

    let balance = client.get_balance(&sender_keypair.pubkey())?;
    println!("Sender's balance: {} lamports", balance);

    let test_lamports = rand::thread_rng().gen_range(1000..10000);
    let test_sig = send_transaction_async(&client, &sender_keypair, recipient_pubkey, test_lamports).await?;
    println!("Test transaction sent: {}", test_sig);

    println!("Start spamming? (y/n): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if input.trim().to_lowercase() != "y" {
        println!("Spamming cancelled.");
        return Ok(());
    }

    println!("Starting spammer -> to {}", rpc_url);

    loop {
        let loop_start = Instant::now();
        let mut tasks = Vec::new();

        for _ in 0..num_tasks {
            let client_clone = client.clone();
            let sender_clone = Keypair::from_bytes(&private_key_bytes)?;
            let recipient_clone = recipient_pubkey;
            tasks.push(tokio::spawn(async move {
                let lamports = rand::thread_rng().gen_range(1000..10000);
                match send_transaction_async(&client_clone, &sender_clone, recipient_clone, lamports).await {
                    Ok(sig) => (),
                    Err(e) => eprintln!("Failed: {}", e),
                }
            }));
        }

        for task in tasks {
            let _ = task.await;
        }

        let elapsed = loop_start.elapsed().as_millis() as u64;
        println!("Transactions sent in {} ms", elapsed);
    }
}

async fn send_transaction_async(
    client: &RpcClient,
    from: &Keypair,
    to: Pubkey,
    lamports: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let instruction = system_instruction::transfer(&from.pubkey(), &to, lamports);
    let message = Message::new(&[instruction], Some(&from.pubkey()));
    let blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new(&[from], message, blockhash);
    let signature = client.send_transaction(&transaction)?;
    Ok(signature.to_string())
}