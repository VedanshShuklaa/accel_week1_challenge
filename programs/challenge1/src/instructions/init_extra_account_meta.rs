use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta,
    state::ExtraAccountMetaList,
    seeds::Seed,
};

use crate::{state::VaultState};

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    /// payer of the init rent — this is the user in your flow
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        // we know we will store two metas (vault_state + whitelist)
        // if you plan more metas, increase this constant accordingly
        space = ExtraAccountMetaList::size_of(2).unwrap(),
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    /// existing or newly-created vault_state account (depends on flow)
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    /// Build the list of ExtraAccountMeta using the accounts in `self`.
    /// Must be called from the instruction handler after Anchor has populated
    /// `self` (i.e., can't be used inside the attribute).
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        // Compute the whitelist PDA for (vault_state, payer)
        let whitelist_meta = ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal { bytes: b"whitelist".to_vec() },
                Seed::AccountKey { index: 3 },
                Seed::AccountKey { index: 4 }
            ],
            false,
            false,
        ).unwrap();

        // Build the list of ExtraAccountMeta entries.
        // There are multiple constructors in the extra-account-metas crate:
        //  - `new_with_seeds(...)` if you want the resolver to re-derive the PDA from seeds
        //  - `new_pubkey(...)` if you want to provide the concrete pubkey
        //
        // Use whichever constructor the crate exposes. Below I show both patterns as examples.

        // Example 1: add a meta that is described by seeds (preferred when the resolver must re-derive).
        let vault_meta = ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal { bytes: b"vault_state".to_vec() },
                // If the crate expects `AccountKey { index }`, pick the index of the account used to reference the other account,
                // or replace with Seed::AccountPubkey { pubkey: self.payer.key() } if that constructor exists.
                Seed::AccountKey { index: 3 }, // <-- adjust index as required by the crate
            ],
            // is_signer, is_writable
            false,
            true,
        ).unwrap();

        Ok(vec![vault_meta, whitelist_meta])
    }
}
