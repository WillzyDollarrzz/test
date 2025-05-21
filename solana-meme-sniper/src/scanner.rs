use anyhow::Result;
use futures::stream::StreamExt;
use solana_client::pubsub_client::{PubsubClient, RpcTransactionLogsFilter};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::str::FromStr;

pub struct TokenScanner {
    ws_url: String,
    program_id: Pubkey,
}

impl TokenScanner {
    pub fn new(ws_url: String, program_id: Pubkey) -> Self {
        Self { ws_url, program_id }
    }

    pub async fn stream_events(&self) -> Result<impl futures::Stream<Item = (Pubkey, Pubkey, u64)>> {
        let (_c, mut rx) = PubsubClient::logs_subscribe(
            &self.ws_url,
            RpcTransactionLogsFilter::All,
            CommitmentConfig::confirmed(),
        )?;
        let pid = self.program_id.to_string();
        Ok(rx.filter_map(move |msg| {
            if msg.value.logs.iter().any(|l| l.contains(&pid) && l.contains("Instruction: Create")) {
                let keys = &msg.value.transaction.message.account_keys;
                if keys.len() >= 3 {
                    if let (Ok(m), Ok(c)) = (Pubkey::from_str(&keys[1]), Pubkey::from_str(&keys[2])) {
                        return Some((m, c, msg.context.slot));
                    }
                }
            }
            None
        }))
    }
}
