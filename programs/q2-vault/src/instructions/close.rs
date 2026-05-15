use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::derive_account::Close;

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(System::id(), accounts);

        transfer(cpi_ctx, self.vault.lamports())
    }
}
