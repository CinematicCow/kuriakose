use anchor_lang::prelude::*;

declare_id!("7Hamd5qWYGpTicYuTLmaZrWcZL77Z8JnGboDWbFCNLxF");

#[program]
pub mod kuriakose {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
