use anchor_lang::prelude::*;

use crate::{derive_account::Init, InitBumps};

impl<'info> Init<'info> {
    pub fn init(&mut self, bumps: &InitBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}
