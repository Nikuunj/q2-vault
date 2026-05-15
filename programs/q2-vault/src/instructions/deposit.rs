use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::derive_account::Deposit;

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(System::id(), accounts);

        transfer(cpi_ctx, amount)
    }
}
