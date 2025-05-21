use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use solana_meme_sniper::{
    cli::{App, Args},
    config::Config,
    crypto::load_or_generate_keypair,
    runner::start_bot,
};
use std::{process, sync::Arc};
use tokio::sync::Mutex;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();
    let config = Arc::new(Mutex::new(
        Config::from_env().unwrap_or_else(|e| { eprintln!("{}", e); process::exit(1) })
    ));
    let keypair = load_or_generate_keypair(&args.keypair_path)
        .unwrap_or_else(|e| { eprintln!("{}", e); process::exit(1) });

    let app = App::new(args.output_format);
    info!("Starting Solana Meme Sniper Bot");
    if let Err(err) = start_bot(config, keypair, app).await {
        error!("Bot failed: {}", err);
        process::exit(1);
    }
    Ok(())
}
