use anyhow::Result;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::{env, str::FromStr};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub ws_url: String,
    pub trade_amount_usd: f64,
    pub target_profit_percentage: f64,
    pub stop_loss_percentage: f64,
    pub min_market_cap: f64,
    pub max_market_cap: f64,
    pub min_liquidity_usd: f64,
    pub pump_fun_program_id: Pubkey,
    pub pyth_price_account: Pubkey,
    pub max_trades: usize,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let get_s = |k, d: &str| env::var(k).unwrap_or_else(|_| d.into());
        let get_f = |k, d| get_s(k,d).parse::<f64>();
        let rpc_url = get_s("SOLANA_RPC_URL","");
        let ws_url  = get_s("SOLANA_WS_URL","");
        Url::parse(&rpc_url)?;
        Url::parse(&ws_url.replace("wss://","https://"))?;

        Ok(Config {
            rpc_url,
            ws_url,
            trade_amount_usd: get_f("TRADE_AMOUNT_USD","160")?,
            target_profit_percentage: get_f("TARGET_PROFIT_PERCENTAGE","11")?,
            stop_loss_percentage: get_f("STOP_LOSS_PERCENTAGE","20")?,
            min_market_cap: get_f("MIN_MARKET_CAP","3000")?,
            max_market_cap: get_f("MAX_MARKET_CAP","5000")?,
            min_liquidity_usd: get_f("MIN_LIQUIDITY_USD","1000")?,
            pump_fun_program_id: Pubkey::from_str(&get_s("PUMP_FUN_PROGRAM_ID",""))?,
            pyth_price_account: Pubkey::from_str(&get_s("PYTH_SOL_PRICE_ACCOUNT",""))?,
            max_trades: env::var("MAX_TRADES")?.parse::<usize>()?,
        })
    }
}

