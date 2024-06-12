use anchor_lang::prelude::*;

declare_id!("DfTSpTzBFZgwPTJH8cZfKiqCd5sP2VoxJgCjgp8funZ1");

#[program]
pub mod smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
