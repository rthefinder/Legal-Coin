use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeMint, MintTo, SetAuthority, Token, TokenAccount, Mint};

declare_id!("LEGAL111111111111111111111111111111111111111");

#[program]
pub mod legal {
    use super::*;

    pub fn launch(
        ctx: Context<Launch>,
        decimals: u8,
        amount: u64,
    ) -> Result<()> {
        // Initialize mint with the program PDA as authority, then mint all supply
        token::initialize_mint(
            ctx.accounts.into_init_mint_context(),
            decimals,
            &ctx.accounts.program_authority.key(),
        )?;

        token::mint_to(
            ctx.accounts.into_mint_to_context(),
            amount,
        )?;

        // Revoke mint authority (set to None)
        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            token::AuthorityType::MintTokens,
            None,
        )?;

        // Revoke freeze authority as well
        token::set_authority(
            ctx.accounts.into_set_freeze_context(),
            token::AuthorityType::FreezeAccount,
            None,
        )?;

        emit!(LaunchEvent {
            mint: ctx.accounts.mint.key(),
            creator: ctx.accounts.creator.key(),
            amount,
            decimals,
            ts: Clock::get()?.unix_timestamp,
        });

        emit!(AuthorityRevoked { mint: ctx.accounts.mint.key() });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Launch<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: mint uninitialized account provided by caller
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    /// CHECK: program authority PDA
    pub program_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Launch<'info> {
    fn into_init_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, InitializeMint<'info>> {
        let cpi_accounts = InitializeMint {
            mint: self.mint.to_account_info().clone(),
            rent: self.rent.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }

    fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info().clone(),
            to: self.destination.to_account_info().clone(),
            authority: self.program_authority.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }

    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.mint.to_account_info().clone(),
            current_authority: self.program_authority.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }

    fn into_set_freeze_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.mint.to_account_info().clone(),
            current_authority: self.program_authority.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

#[event]
pub struct LaunchEvent {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub amount: u64,
    pub decimals: u8,
    pub ts: i64,
}

#[event]
pub struct AuthorityRevoked {
    pub mint: Pubkey,
}

#[event]
pub struct TradingEnabled {
    pub mint: Pubkey,
}

#[error_code]
pub enum LegalError {
    #[msg("Post-launch modifications are forbidden")]
    ForbiddenPostLaunch,
}
