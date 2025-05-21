use anyhow::Result;
use anchor_client::{Client, Cluster};
use solana_sdk::pubkey::Pubkey;
use pump_fun::accounts::BondingCurve;
use crate::{config::Config, oracle::get_sol_price};

pub struct PoolInfo {
    pub mint: Pubkey,
    pub curve: Pubkey,
    pub price_in_sol: f64,
    pub market_cap: f64,
    pub liquidity: f64,
}

pub async fn fetch_pool_info(cfg: &Config, mint: Pubkey, curve: Pubkey) -> Result<PoolInfo> {
    let payer = anchor_client::solana_sdk::signature::Keypair::new();
    let client = Client::new_with_options(
        Cluster::Custom(cfg.rpc_url.clone(), cfg.rpc_url.clone()),
        payer, Default::default()
    );
    let prog = client.program(cfg.pump_fun_program_id);

    let bc: BondingCurve = prog.account(curve)?;
    let reserve_sol = bc.reserve_sol as f64 / 1e9;
    let supply     = bc.supply     as f64 / 10f64.powi(bc.decimals as i32);
    let price_in_sol = reserve_sol / supply;

    let sol_usd = get_sol_price(&prog.rpc(), cfg).await?;
    let market_cap = price_in_sol * supply * sol_usd;
    let liquidity  = reserve_sol * sol_usd;

    Ok(PoolInfo { mint, curve, price_in_sol, market_cap, liquidity })
}
