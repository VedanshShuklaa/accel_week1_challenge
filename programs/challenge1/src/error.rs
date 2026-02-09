use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Not whitelisted")]
    NotWhitelisted,

    #[msg("Amount exceeds limit")]
    AmountExceedsLimit,

    #[msg("Invalid vault token account")]
    InvalidVaultTokenAccount,

    #[msg("Invalid mint")]
    InvalidMint,

    #[msg("Invalid owner")]
    InvalidOwner,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid Vault")]
    InvalidVault,
}
