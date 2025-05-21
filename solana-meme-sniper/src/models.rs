use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRecord {
    pub trade_id: usize,
    pub timestamp: DateTime<Utc>,
    pub token_mint: Pubkey,
    pub entry_price: f64,
    pub exit_price: Option<f64>,
    pub amount_sol: f64,
    pub profit_loss_sol: Option<f64>,
    pub priority_fee_sol: f64,
    pub outcome: Option<TradeOutcome>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TradeOutcome {
    TakeProfit,
    StopLoss,
    Failed,
}
