use clap::Parser;
use colored::*;
use solana_sdk::pubkey::Pubkey;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(short, long, default_value = "text")]
    pub output_format: OutputFormat,
    #[clap(short, long, parse(from_os_str))]
    pub keypair_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat { Text, Json }

impl std::str::FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

pub struct App {
    pub format: OutputFormat,
    pub trade_count: Arc<Mutex<usize>>,
    pub wins: Arc<Mutex<usize>>,
    pub losses: Arc<Mutex<usize>>,
    pub total_profit_sol: Arc<Mutex<f64>>,
    pub total_fees_sol: Arc<Mutex<f64>>,
}

impl App {
    pub fn new(format: OutputFormat) -> Self {
        Self {
            format,
            trade_count: Arc::new(Mutex::new(0)),
            wins: Arc::new(Mutex::new(0)),
            losses: Arc::new(Mutex::new(0)),
            total_profit_sol: Arc::new(Mutex::new(0.0)),
            total_fees_sol: Arc::new(Mutex::new(0.0)),
        }
    }

    pub async fn print_startup_banner(&self) {
        println!("{}", "\n============================".bright_blue());
        println!("{}", " anon PUMP.FUN SNIPER BOT".bright_green().bold());
        println!("Version {}", env!("CARGO_PKG_VERSION"));
        println!("{}", "============================\n".bright_blue());
    }

    pub async fn prompt_wallet_setup(&self, pubkey: &Pubkey, show_pk: bool, pk_str: &str) -> bool {
        println!("{}", "\n📝 WALLET SETUP".yellow().bold());
        println!("Address: {}", pubkey.to_string().bright_green());
        if show_pk {
            println!("{}", "⚠️ Save this private key!".red().bold());
            println!("{}", pk_str.bright_red());
        }
        println!("\nHave you funded this wallet with ≥$160 SOL? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        matches!(input.trim().to_lowercase().as_str(), "y"|"yes")
    }

    pub async fn display_token_skipped(&self, mint: &Pubkey, reason: &str) {
        println!("\n⏭️ Skipped {} — {}", mint, reason.red());
    }
    // ... add back all your other display_* methods here verbatim ...
}
