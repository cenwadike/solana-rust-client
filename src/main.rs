use anchor_client;
use blob;
use borsh::{BorshDeserialize, BorshSerialize};
pub use solana_client::rpc_client::RpcClient;
pub use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};
use solana_sdk::{message::Message, signature::Signature, signer::EncodableKey};

#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "borsh")]
pub struct Initialize {}

#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "borsh")]
pub struct UpdateBlob {
    pub data: Vec<u8>,
}

pub fn initialize_program() -> Result<Signature, Box<dyn std::error::Error>> {
    let rpc_connection = RpcClient::new("https://api.devnet.solana.com");
    let program_id = blob::ID;

    let (blob_account, _) = Pubkey::find_program_address(&[&b"blob"[..]], &program_id);

    let payer: Keypair =
        Keypair::read_from_file("/Users/cenwadike/.config/solana/solfate-dev.json")?;

    println!("program id: {:?}", program_id);
    println!("payer: {:?}", payer.pubkey());
    println!("blob account: {:?}", blob_account);

    let instruction_data = Initialize {};

    // set up accounts
    let accounts = vec![
        AccountMeta::new(blob_account, false),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(system_program::id(), false),
    ];

    // get initialize discriminant
    let initialize_discriminant = get_discriminant("global", "initialize");

    // construct instruction
    let ix = Instruction::new_with_borsh(
        program_id.clone(),
        &(initialize_discriminant, instruction_data),
        accounts.clone(),
    );

    // get latest block hash
    let blockhash = rpc_connection
        .get_latest_blockhash()
        .expect("latest blockhash");

    // construct message
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);

    //construct transaction
    let mut tx = Transaction::new_unsigned(msg);

    // sign transaction
    tx.sign(&[&payer], tx.message.recent_blockhash);

    // send and confirm transaction
    let tx_signature = rpc_connection.send_and_confirm_transaction(&tx)?;

    println!(
        "Initialization successful. Tx signature: {:?}",
        tx_signature
    );
    Ok(tx_signature)
}

pub fn update_blob() -> Result<Signature, Box<dyn std::error::Error>> {
    let rpc_connection = RpcClient::new("https://api.devnet.solana.com");
    let program_id = blob::ID;

    let (blob_account, _) = Pubkey::find_program_address(&[&b"blob"[..]], &program_id);

    let payer: Keypair =
        Keypair::read_from_file("/Users/cenwadike/.config/solana/solfate-dev.json")?;

    println!("program id: {:?}", program_id);
    println!("payer: {:?}", payer.pubkey());
    println!("blob account: {:?}", blob_account);

    //  construct instruction data
    let instruction_data = UpdateBlob {
        data: "another data".as_bytes().to_vec(),
    };

    // set up accounts
    let accounts = vec![
        AccountMeta::new(blob_account, false),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(system_program::id(), false),
    ];

    // get discriminant
    let update_blob_discriminant = get_discriminant("global", "update_blob");

    // construct instruction
    let ix = Instruction::new_with_borsh(
        program_id.clone(),
        &(update_blob_discriminant, instruction_data),
        accounts.clone(),
    );

    // get latest block hash
    let blockhash = rpc_connection.get_latest_blockhash()?;

    // construct message
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);

    //construct transaction
    let mut tx = Transaction::new_unsigned(msg);

    // sign transaction
    tx.sign(&[&payer], tx.message.recent_blockhash);

    // send and confirm transaction
    let tx_signature = rpc_connection.send_and_confirm_transaction(&tx)?;

    println!("Update blob successful. Tx signature: {:?}", tx_signature);
    Ok(tx_signature)
}

pub fn get_discriminant(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_client::anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()
            [..8],
    );
    sighash
}

fn main() {
    // initialize_program().expect("error");
    update_blob().expect("error");
}
