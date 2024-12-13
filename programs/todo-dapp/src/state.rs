use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct UserProfile{
    pub authority: Pubkey,
    pub last_todo_count: u8,
    pub todo_count: u8
}

#[account]
#[derive(Default, InitSpace)]
pub struct TodoAccount{
    pub authority: Pubkey,
    pub idx: u8,
    pub marked: bool,
    #[max_len(20)]
    pub content: String,
}


