use anchor_lang::prelude::*;
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("2UqjJpZWHDKFzVP2PjXiaeSEvsnZc6o8p6Kgd3vJ8aVV");

pub mod states;
pub mod constants;
use crate::{states::*,constants::*};

#[program]
pub mod job_board{
    use super::*;

    pub fn initialize_account(ctx: Context<InitializeAccount>, _name: String, _is_employer: bool) -> Result<()>{
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.name = _name;
        user_profile.is_employer = _is_employer;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_job=0;
        user_profile.job_count=0;

        Ok(())
    }

    pub fn create_job_post(ctx : Context<CreateJobPost>, _title: String, _desc: String) -> Result<()>{
        let job_post = &mut ctx.accounts.job_post;
        let user_profile = &mut ctx.accounts.user_profile;
        job_post.title = _title;
        job_post.desc = _desc;
        job_post.idx = user_profile.last_job;
        job_post.status = true;
        user_profile.last_job = user_profile.last_job.checked_add(1).unwrap();
        user_profile.job_count = user_profile.job_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn apply_for_job(ctx : Context<Apply>) -> Result<()>{
        let job_post = &mut ctx.accounts.job_post;
        job_post.applicants.push(ctx.accounts.user_profile.key());
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeAccount<'info>{
    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(init,
             seeds=[USER_TAG, authority.key().as_ref()],
             bump,
             payer=authority,
             space=8 + std::mem::size_of::<User>(),
             )]
    pub user_profile : Box<Account<'info , User>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct CreateJobPost<'info>{
    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(mut,
             seeds=[USER_TAG, authority.key().as_ref()],
             bump,
             has_one=authority)]
    pub user_profile : Box<Account<'info, User>>,

    #[account(init,
              seeds=[JOB_TAG, authority.key().as_ref(), &[user_profile.last_job as u8].as_ref()],
              bump,
              payer=authority,
              space=8 + std::mem::size_of::<JobPost>(),)]
    pub job_post : Box<Account<'info, JobPost>>,

    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
#[instruction()]
pub struct Apply<'info>{
    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(mut,
             seeds=[USER_TAG, authority.key().as_ref()],
             bump,
             has_one=authority)]
    pub user_profile : Box<Account<'info, User>>,

    #[account(mut)]
    pub job_post:Box<Account<'info, JobPost>>,

    pub system_program : Program<'info, System>
}



