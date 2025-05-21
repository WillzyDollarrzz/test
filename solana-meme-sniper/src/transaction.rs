use anyhow::Result;
use anchor_client::{Client, Cluster};
use pump_fun::accounts::{Buy as BuyAcc, Sell as SellAcc};
use pump_fun::instruction::{Buy as BuyArgs, Sell as SellArgs};
use solana_sdk::{signature::{Keypair, Signature}, transaction::Transaction, commitment_config::CommitmentConfig};
use crate::{config::Config, pool::PoolInfo};

pub async fn send_buy(cfg: &Config, payer: &Keypair, pool: &PoolInfo) -> Result<(f64, Signature)> {
    let client = Client::new_with_options(
        Cluster::Custom(cfg.rpc_url.clone(), cfg.rpc_url.clone()),
        payer.clone(),
        CommitmentConfig::confirmed(),
    );
    let prog = client.program(cfg.pump_fun_program_id);
    let ux_amt = ((cfg.trade_amount_usd / pool.price_in_sol) * 1e9) as u64;

    let ix = prog.request()
        .accounts(BuyAcc {
            global: /* fill from pump_fun.rs PDA */,
            fee_recipient: /* PDA */,
            mint: pool.mint,
            bonding_curve: pool.curve,
            associated_bonding_curve: /* PDA */,
            associated_user: /* PDA */,
            user: payer.pubkey(),
            system_program: solana_sdk::system_program::ID,
            token_program: spl_token::ID,
            creator_vault: /* PDA */,
            event_authority: /* PDA */,
            program: prog.id(),
        })
        .args(BuyArgs { amount: ux_amt, max_sol_cost: ux_amt })
        .instructions()?;

    let recent = prog.rpc().get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(&ix, Some(&payer.pubkey()), &[payer], recent);
    let sig = prog.rpc().send_and_confirm_transaction(&tx)?;
    Ok((ux_amt as f64 / 1e9, sig))
}

pub async fn send_sell(cfg: &Config, payer: &Keypair, pool: &PoolInfo, amount: f64) -> Result<Signature> {
    let client = Client::new_with_options(
        Cluster::Custom(cfg.rpc_url.clone(), cfg.rpc_url.clone()),
        payer.clone(),
        CommitmentConfig::confirmed(),
    );
    let prog = client.program(cfg.pump_fun_program_id);
    let ux_amt = (amount * 1e9) as u64;

    let ix = prog.request()
        .accounts(SellAcc {
            global: /* PDA */,
            user: payer.pubkey(),
            /* etc… */
        })
        .args(SellArgs { amount: ux_amt })
        .instructions()?;

    let recent = prog.rpc().get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(&ix, Some(&payer.pubkey()), &[payer], recent);
    Ok(prog.rpc().send_and_confirm_transaction(&tx)?)
}
