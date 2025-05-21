use anyhow::Result;
use bs58;
use solana_sdk::signature::{Keypair, read_keypair_file};
use std::{env, fs, io::Write, path::Path, path::PathBuf};

pub fn load_or_generate_keypair(path: &Option<PathBuf>) -> Result<Keypair> {
    if let Some(p) = path { return read_keypair_file(p).map_err(|e| e.into()); }
    if let Ok(pk) = env::var("PRIVATE_KEY") {
        let bytes = bs58::decode(pk).into_vec()?;
        return Keypair::from_bytes(&bytes).map_err(|e| e.into());
    }
    let kp = Keypair::new();
    let sk = bs58::encode(kp.to_bytes()).into_string();
    let mut content = if Path::new(".env").exists() {
        fs::read_to_string(".env")?
    } else { String::new() };
    content.push_str(&format!("\nPRIVATE_KEY={}\n", sk));
    fs::write(".env", content)?;
    Ok(kp)
}
