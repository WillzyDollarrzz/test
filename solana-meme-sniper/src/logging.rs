use crate::models::TradeRecord;
use anyhow::Result;
use std::{fs::File, io::Write};

pub fn save_trade_history(records: &[TradeRecord]) -> Result<()> {
    let ts = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("trade_history_{}.json", ts);
    let json = serde_json::to_string_pretty(records)?;
    let mut file = File::create(&filename)?;
    file.write_all(json.as_bytes())?;
    println!("Saved history to {}", filename);
    Ok(())
}
