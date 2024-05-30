use blob;
pub use solana_client::rpc_client::RpcClient;
pub use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
    transaction_context,
};
use solana_sdk::{message::Message, signature::Signature};

pub fn signed_call() -> Result<Signature, Box<dyn std::error::Error>> {
    let rpc_connection = RpcClient::new("https://api.devnet.solana.com");
    let program_id = Pubkey::new_unique(); //TODO: use correct program id
    let (blob_account, _) = Pubkey::find_program_address(&[&b"blob"[..]], &program_id);
    let payer = Keypair::new();

    // request 5 sol for payer 
    request_airdrop(&payer, &rpc_connection, 5)?;

    // construct instruction data
    let instruction_data = blob::instruction::UpdateBlob {
        data: "new data".to_string(),
    };

    // set up accounts
    let accounts = vec![
        AccountMeta::new(blob_account, false),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(system_program::ID, false),
    ];

    // construct instruction
    let ix =
        Instruction::new_with_bincode(program_id.clone(), &instruction_data.data, accounts.clone());

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
    let tx_signature = rpc_connection
        .send_and_confirm_transaction(&tx)
        .expect("transaction successful");

    Ok(tx_signature)
}

/// Requests that AMOUNT lamports are transfered to PAYER via a RPC
/// call over CONNECTION.
///
/// Airdrops are only avaliable on test networks.
pub fn request_airdrop(payer: &Keypair, connection: &RpcClient, amount: u64) -> Result<(), Box<dyn std::error::Error>> {
    let sig = connection.request_airdrop(&payer.pubkey(), amount)?;
    assert!(connection.confirm_transaction(&sig)?);
    Ok(())
}

fn main() {
    println!("Hello, world!");

    // call signed_call function
    signed_call().unwrap();
}
