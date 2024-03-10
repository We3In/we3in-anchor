use anchor_lang::prelude::*;

declare_id!("7cBTgEf3vCgNqqp63wNAPFhXK2MDAsRz7co4kCtXJuNp");

#[program]
pub mod contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
