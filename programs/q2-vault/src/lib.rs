use anchor_lang::prelude::*;

declare_id!("AakuWqCs6Ud8S3LbWW9AJ5gSdPgTX1DsmZcGEW4irqGG");

mod derive_account;
mod instructions;
mod states;

pub use derive_account::*;
pub use instructions::*;
pub use states::*;

#[program]
pub mod q2_vault {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}
