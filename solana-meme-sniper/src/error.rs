use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Insufficient balance: {0} < {1}")]
    InsufficientBalance(f64, f64),
    #[error("Market cap out of range: {0}")]
    MarketCapOutOfRange(f64),
    #[error("Low liquidity: {0}")]
    LowLiquidity(f64),
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    #[error("RPC error: {0}")]
    RpcError(String),
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    #[error("Max trades reached")]
    MaxTradesReached,
}
