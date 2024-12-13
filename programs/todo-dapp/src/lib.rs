use anchor_lang::prelude::*;

declare_id!("JBKmhoLk1YFT7s4kUyE4jhvyYB8UL4PekVjpx7jPvjfx");

#[program]
pub mod todo_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
