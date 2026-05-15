use anchor_lang::prelude::*;

declare_id!("AakuWqCs6Ud8S3LbWW9AJ5gSdPgTX1DsmZcGEW4irqGG");

#[program]
pub mod q2_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
