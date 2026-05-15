use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::Withdraw;

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_acc = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(System::id(), cpi_acc, signer_seeds);

        transfer(cpi_ctx, amount)
    }
}
