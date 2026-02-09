use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::VaultState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // Token-2022 mint
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = user,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"vault_state", user.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    /// CHECK: PDA authority for vault
    #[account(
        seeds = [b"vault-authority", vault_state.key().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
        associated_token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.set_inner(VaultState {
            authority: self.user.key(),
            mint: self.mint.key(),
            vault_token_account: self.vault_token_account.key(),
            vault_state_bump: bumps.vault_state,
            vault_authority_bump: bumps.vault_authority,
        });
        Ok(())
    }
}