use anchor_lang::prelude::*;
use crate::{state::VaultState, state::WhitelistEntry};
use anchor_spl::token_interface::{
    TokenAccount,
    TokenInterface,
    Mint,
    TransferChecked,
    transfer_checked,
};

use crate::error::VaultError;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds=[b"whitelist", vault_state.key().as_ref(), user.key().as_ref()],
        bump = whitelist.bump,
        constraint = whitelist.vault == vault_state.key()
    )]
    pub whitelist: Account<'info, WhitelistEntry>,

    #[account(
        mut,
        constraint = user_token_account.mint == vault_state.mint @ VaultError::InvalidMint,
        constraint = user_token_account.owner == user.key() @ VaultError::InvalidOwner,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: PDA authority for vault
    #[account(
        seeds=[b"vault-authority", vault_state.key().as_ref()],
        bump = vault_state.vault_authority_bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = vault_token_account.key() == vault_state.vault_token_account @ VaultError::InvalidVaultTokenAccount,
        constraint = vault_token_account.mint == vault_state.mint @ VaultError::InvalidMint,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&self, amount: u64) -> Result<()> {
        require!(amount <= self.whitelist.max_amount, VaultError::AmountExceedsLimit);
        require!(amount > 0, VaultError::InvalidAmount);

        let vault_state = self.vault_state.key();

        let seeds = [
            b"vault-authority",
            vault_state.as_ref(),
            &[self.vault_state.vault_authority_bump],
        ];

        let cpi_accounts = TransferChecked {
            from: self.vault_token_account.to_account_info(),
            to: self.user_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.vault_authority.to_account_info(),
        };
        
        let binding = [seeds.as_ref()];

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            &binding
        );

        transfer_checked(cpi_context, amount, 6)?;

        Ok(())
    }
}