use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, native_token::LAMPORTS_PER_SOL};

pub fn check_wallet_balance(rpc: &RpcClient, pubkey: &Pubkey) -> Result<f64> {
    let lamports = rpc.get_balance(pubkey)?;
    Ok(lamports as f64 / LAMPORTS_PER_SOL as f64)
}
