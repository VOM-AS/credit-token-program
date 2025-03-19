use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

use solana_security_txt::security_txt;

#[security_txt(
    name = "Credit Token Program",
    project_url = "https://github.com/yourusername/credit-token-program",
    contacts = "email:security@yourdomain.com,github:https://github.com/yourusername/credit-token-program/security",
    policy = "https://github.com/yourusername/credit-token-program/blob/main/SECURITY.md",
    preferred_languages = "en",
    source_code = "https://github.com/yourusername/credit-token-program",
    source_release = "v0.1.0",
    source_revision = "main",
    auditors = "None",
    acknowledgements = "None reported yet"
)]

declare_id!("HJWf9ne42BeCtCWti7VmydDs9jgnZxiTtMUBJTxb29g5");

#[program]
pub mod credit_token {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        // Store the program authority (owner) in the vault account
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn purchase_credits(ctx: Context<PurchaseCredits>, credit_amount: u64) -> Result<()> {
        let user = &ctx.accounts.user;
        let vault = &ctx.accounts.vault;
        
        // Calculate the cost based on the amount
        let cost = match credit_amount {
            100 => 0.02 * LAMPORTS_PER_SOL as f64,
            400 => 0.09 * LAMPORTS_PER_SOL as f64,
            1000 => 0.2 * LAMPORTS_PER_SOL as f64,
            5000 => 0.94 * LAMPORTS_PER_SOL as f64,
            _ => return Err(ErrorCode::InvalidAmount.into()),
        } as u64;
        
        // Transfer SOL from user to vault
        let ix = system_instruction::transfer(
            &user.key(),
            &vault.key(),
            cost,
        );
        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                user.to_account_info(),
                vault.to_account_info(),
            ],
        )?;
        
        // Payment is complete. Credit tracking is handled by the backend.
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        
        // Check if there's enough SOL in the vault
        require!(
            vault.to_account_info().lamports() >= amount,
            ErrorCode::InsufficientFunds
        );

        // Verify the authority
        require!(
            ctx.accounts.authority.key() == vault.authority,
            ErrorCode::UnauthorizedWithdrawal
        );

        // Transfer SOL from vault to authority
        **vault.to_account_info().try_borrow_mut_lamports()? = vault
            .to_account_info()
            .lamports()
            .checked_sub(amount)
            .ok_or(ErrorCode::InsufficientFunds)?;

        **ctx.accounts.authority.try_borrow_mut_lamports()? = ctx
            .accounts.authority
            .lamports()
            .checked_add(amount)
            .ok_or(ErrorCode::OverflowError)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32, // discriminator + pubkey
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseCredits<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, VaultAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultAccount {
    pub authority: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
    #[msg("Unauthorized withdrawal attempt")]
    UnauthorizedWithdrawal,
    #[msg("Arithmetic overflow")]
    OverflowError,
    #[msg("Invalid credit amount")]
    InvalidAmount,
}
