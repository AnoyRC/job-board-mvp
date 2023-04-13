use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct User{
    pub name : String,
    pub is_employer : bool,
    pub authority : Pubkey,
    pub last_job : u8,
    pub job_count : u8,
}

#[account]
#[derive(Default)]
pub struct JobPost{
    pub title : String,
    pub desc : String,
    pub idx : u8,
    pub status : bool,
    pub applicants : Vec<Pubkey>,
    pub authority : Pubkey,
}