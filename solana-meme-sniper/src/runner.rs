use crate::{cli::App, config::Config, crypto::load_or_generate_keypair, scanner::TokenScanner, trade::TradeExecutor, wallet::check_wallet_balance, oracle::get_sol_price};
use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Keypair;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::StreamExt;

pub async fn start_bot(
    config: Arc<Mutex<Config>>,
    keypair: Keypair,
    app: App,
) -> Result<()> {
    app.print_startup_banner().await;
    let cfg = config.lock().await.clone();
    let rpc_client = RpcClient::new_with_commitment(cfg.rpc_url.clone(), CommitmentConfig::confirmed());
    let scanner = TokenScanner::new(cfg.ws_url.clone(), cfg.pump_fun_program_id);
    let executor = TradeExecutor::new(cfg.clone(), keypair.clone());

    let mut events = scanner.stream_events().await?;
    while let Some((mint, curve, slot)) = events.next().await {
        // enforce 1.9s cutoff
        match rpc_client.get_block_time(slot) {
            Ok(bt) => {
                let age = chrono::Utc::now().timestamp() - bt;
                if age as f64 > 1.9 {
                    app.display_token_skipped(&mint, &format!("{}s old", age)).await;
                    continue;
                }
            }
            Err(e) => {
                app.display_token_skipped(&mint, &format!("no block time: {}", e)).await;
                continue;
            }
        }
        if let Err(e) = executor.execute(mint, curve).await {
            app.display_token_skipped(&mint, &e.to_string()).await;
        }
    }
    Ok(())
}
