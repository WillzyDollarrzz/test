use anyhow::Result;
use pyth_sdk_solana::PriceFeed;
use solana_client::rpc_client::RpcClient;
use crate::config::Config;

pub async fn get_sol_price(rpc: &RpcClient, cfg: &Config) -> Result<f64> {
    let acct = rpc.get_account(&cfg.pyth_price_account)?;
    let feed = PriceFeed::new(&acct.data);
    let p = feed.get_current_price().ok_or_else(|| anyhow::anyhow!("no price"))?;
    Ok(p.price as f64 * 10f64.powi(p.expo))
}
