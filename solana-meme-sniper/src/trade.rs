use anyhow::Result;
use solana_sdk::signature::Keypair;
use crate::{config::Config, pool::fetch_pool_info, strategy::check_eligibility, transaction::{send_buy, send_sell}};

pub struct TradeExecutor {
    cfg: Config,
    payer: Keypair,
}

impl TradeExecutor {
    pub fn new(cfg: Config, payer: Keypair) -> Self {
        Self { cfg, payer }
    }

    pub async fn execute(&self, mint: solana_sdk::pubkey::Pubkey, curve: solana_sdk::pubkey::Pubkey) -> Result<()> {
        let pool = fetch_pool_info(&self.cfg, mint, curve).await?;
        check_eligibility(&pool, &self.cfg)?;
        let (tokens, buy_sig) = send_buy(&self.cfg, &self.payer, &pool).await?;
        println!("✅ Bought {} tokens (tx={})", tokens, buy_sig);

        loop {
            let info = fetch_pool_info(&self.cfg, mint, curve).await?;
            let gain = (info.price_in_sol - pool.price_in_sol) / pool.price_in_sol * 100.0;
            if gain >= self.cfg.target_profit_percentage || gain <= -self.cfg.stop_loss_percentage {
                let sell_sig = send_sell(&self.cfg, &self.payer, &pool, tokens).await?;
                println!(
                    "{} at {:.2}% — tx={}",
                    if gain>0 { "🎯 TP hit" } else { "🚨 SL hit" },
                    gain, sell_sig
                );
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
        Ok(())
    }
}
