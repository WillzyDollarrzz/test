use anyhow::{Result, bail};
use crate::{pool::PoolInfo, config::Config};

pub fn check_eligibility(pool: &PoolInfo, cfg: &Config) -> Result<()> {
    if pool.market_cap < cfg.min_market_cap || pool.market_cap > cfg.max_market_cap {
        bail!("MC ${:.0} outside [{:.0},{:.0}]", pool.market_cap, cfg.min_market_cap, cfg.max_market_cap);
    }
    if pool.liquidity < cfg.min_liquidity_usd {
        bail!("Liquidity ${:.0} < ${:.0}", pool.liquidity, cfg.min_liquidity_usd);
    }
    Ok(())
}
