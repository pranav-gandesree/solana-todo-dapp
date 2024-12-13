use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod state;

#[allow(unused_imports)]
use crate::{constants::*, state::*, error::*};


declare_id!("JBKmhoLk1YFT7s4kUyE4jhvyYB8UL4PekVjpx7jPvjfx");

#[program]
pub mod todo_dapp {
    use super::*;

    //initialise user
    pub fn initialize_user(
        ctx: Context<InitializeUser>
    ) -> Result<()> {
        //initialise user profile and put default values
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_todo_count = 0;
        user_profile.todo_count =0;

    Ok(())
    }

    //add a todo
    pub fn add_todo(
        ctx: Context<AddTodo>,
        _content: String
    ) -> Result<()>{
        //initialize variables
        let todo_account = &mut ctx.accounts.todo_account;
        let user_profile = &mut ctx.accounts.user_profile;
        
        //fill the todo account with proper values
        todo_account.authority = ctx.accounts.authority.key();
        todo_account.idx = user_profile.last_todo_count;
        todo_account.content = _content;
        todo_account.marked = false;

        //increase todo idx for pda
        user_profile.last_todo_count = user_profile.last_todo_count
        .checked_add(1)
        .unwrap();

        user_profile.todo_count = user_profile.todo_count
        .checked_add(1)
        .unwrap();

    Ok(())
    }

    //mark todo

    //remove todo
}





#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info>{
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>()
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
#[instruction()]
pub struct AddTodo<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [TODO_TAG, authority.key().as_ref(), &[user_profile.last_todo_count as u8].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<TodoAccount>() + 8,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}