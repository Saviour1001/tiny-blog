use anchor_lang::prelude::*;
use light_sdk::{
    compressed_account::LightAccount,
    light_account,
    light_accounts,
    light_program,
    merkle_context::PackedAddressMerkleContext,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[light_program]
#[program]
pub mod myjournal {
    use super::*;

    pub fn create_entry<'info>(
        ctx: LightContext<'_, '_, '_, 'info, CreateEntry<'info>>,
        title: String,
        message: String
    ) -> Result<()> {
        msg!("Creating blog entry");

        ctx.light_accounts.blog_entry.owner = ctx.accounts.signer.key();
        ctx.light_accounts.blog_entry.title = title;
        ctx.light_accounts.blog_entry.message = message;

        Ok(())
    }

    pub fn update_entry<'info>(
        ctx: LightContext<'_, '_, '_, 'info, UpdateEntry<'info>>,
        title: String,
        message: String
    ) -> Result<()> {
        msg!("Updating blog entry");
        if ctx.light_accounts.blog_entry.owner != ctx.accounts.signer.key() {
            return Err(CustomError::Unauthorized.into());
        }

        ctx.light_accounts.blog_entry.title = title;
        ctx.light_accounts.blog_entry.message = message;

        Ok(())
    }

    pub fn delete_entry<'info>(
        ctx: LightContext<'_, '_, '_, 'info, DeleteEntry<'info>>,
        title: String
    ) -> Result<()> {
        msg!("Deleting {} blog entry", title);
        if ctx.light_accounts.blog_entry.owner != ctx.accounts.signer.key() {
            return Err(CustomError::Unauthorized.into());
        }

        Ok(())
    }
}

#[light_account]
#[derive(Clone, Debug, Default)]
pub struct BlogEntryState {
    #[truncate]
    pub owner: Pubkey,
    pub title: String,
    pub message: String,
}

#[error_code]
pub enum CustomError {
    #[msg("No authority to perform this action")]
    Unauthorized,
    #[msg("Invalid blog entry")]
    InvalidBlogEntry,
    #[msg("Blog entry not found")]
    BlogEntryNotFound,
}

#[light_accounts]
#[instruction(title: String, message: String)]
pub struct CreateEntry<'info> {
    #[account(mut)]
    #[fee_payer]
    pub signer: Signer<'info>,
    #[self_program]
    pub self_program: Program<'info, crate::program::Myjournal>,
    /// CHECK: Checked in light-system-program.
    #[authority]
    pub cpi_signer: AccountInfo<'info>,
    #[light_account(init, seeds = [b"BLOG", title.as_bytes(), signer.key().as_ref()])]
    pub blog_entry: LightAccount<BlogEntryState>,
}

#[light_accounts]
#[instruction(title: String, message: String)]
pub struct UpdateEntry<'info> {
    #[account(mut)]
    #[fee_payer]
    pub signer: Signer<'info>,
    #[self_program]
    pub self_program: Program<'info, crate::program::Myjournal>,
    /// CHECK: Checked in light-system-program.
    #[authority]
    pub cpi_signer: AccountInfo<'info>,
    #[light_account(init, seeds = [b"BLOG", title.as_bytes(), signer.key().as_ref()])]
    pub blog_entry: LightAccount<BlogEntryState>,
}

#[light_accounts]
#[instruction(title:String)]
pub struct DeleteEntry<'info> {
    #[account(mut)]
    #[fee_payer]
    pub signer: Signer<'info>,
    #[self_program]
    pub self_program: Program<'info, crate::program::Myjournal>,
    /// CHECK: Checked in light-system-program.
    #[authority]
    pub cpi_signer: AccountInfo<'info>,
    #[light_account(init, seeds = [b"BLOG", title.as_bytes(), signer.key().as_ref()])]
    pub blog_entry: LightAccount<BlogEntryState>,
}
