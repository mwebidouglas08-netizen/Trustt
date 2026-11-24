use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

declare_id!("TRusT11111111111111111111111111111111111111");

/// Maximum fee in basis points (1% = 100 bps). Enforced in config init/update flows by app policy; program stores and validates <= MAX_FEE_BPS.
pub const MAX_FEE_BPS: u16 = 100;

#[program]
pub mod trustlayer_escrow {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        usdc_mint: Pubkey,
        fee_bps: u16,
        fee_recipient: Pubkey,
        arbitration_authority: Pubkey,
    ) -> Result<()> {
        require!(fee_bps <= MAX_FEE_BPS, EscrowError::FeeTooHigh);
        let cfg = &mut ctx.accounts.config;
        cfg.authority = ctx.accounts.authority.key();
        cfg.usdc_mint = usdc_mint;
        cfg.fee_bps = fee_bps;
        cfg.fee_recipient = fee_recipient;
        cfg.arbitration_authority = arbitration_authority;
        cfg.paused = false;
        cfg.bump = ctx.bumps.config;
        Ok(())
    }

    pub fn create_escrow(
        ctx: Context<CreateEscrow>,
        escrow_id: [u8; 32],
        seller: Pubkey,
        amount: u64,
        agreement_hash: [u8; 32],
        deadline_unix: i64,
    ) -> Result<()> {
        require!(!ctx.accounts.config.paused, EscrowError::Paused);
        require!(amount > 0, EscrowError::InvalidAmount);
        require!(seller != ctx.accounts.buyer.key(), EscrowError::BuyerSellerSame);
        require!(ctx.accounts.usdc_mint.key() == ctx.accounts.config.usdc_mint, EscrowError::InvalidMint);
        require!(deadline_unix > Clock::get()?.unix_timestamp, EscrowError::InvalidDeadline);

        let escrow = &mut ctx.accounts.escrow;
        escrow.version = 1;
        escrow.escrow_id = escrow_id;
        escrow.buyer = ctx.accounts.buyer.key();
        escrow.seller = seller;
        escrow.mint = ctx.accounts.usdc_mint.key();
        escrow.token_program = ctx.accounts.token_program.key();
        escrow.vault = ctx.accounts.vault.key();
        escrow.amount = amount;
        escrow.fee_bps = ctx.accounts.config.fee_bps;
        escrow.fee_recipient = ctx.accounts.config.fee_recipient;
        escrow.agreement_hash = agreement_hash;
        escrow.deadline_unix = deadline_unix;
        escrow.state = EscrowState::Created as u8;
        escrow.bump = ctx.bumps.escrow;
        escrow.vault_authority_bump = ctx.bumps.vault_authority;
        Ok(())
    }

    pub fn fund_escrow(ctx: Context<FundEscrow>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;
        require!(escrow.state == EscrowState::Created as u8, EscrowError::InvalidState);
        require!(ctx.accounts.buyer.key() == escrow.buyer, EscrowError::Unauthorized);
        require!(ctx.accounts.buyer_token_account.owner == escrow.buyer, EscrowError::Unauthorized);
        require!(ctx.accounts.buyer_token_account.mint == escrow.mint, EscrowError::InvalidMint);
        require!(ctx.accounts.vault.key() == escrow.vault, EscrowError::InvalidVault);
        require!(ctx.accounts.vault.mint == escrow.mint, EscrowError::InvalidMint);

        let cpi_accounts = TransferChecked {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            mint: ctx.accounts.usdc_mint.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer_checked(cpi_ctx, escrow.amount, ctx.accounts.usdc_mint.decimals)?;

        let escrow = &mut ctx.accounts.escrow;
        escrow.state = EscrowState::Funded as u8;
        escrow.funded_at_unix = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn mark_delivery_submitted(ctx: Context<MarkDelivery>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        require!(
            escrow.state == EscrowState::Funded as u8 || escrow.state == EscrowState::DeliverySubmitted as u8,
            EscrowError::InvalidState
        );
        require!(ctx.accounts.seller.key() == escrow.seller, EscrowError::Unauthorized);
        escrow.state = EscrowState::DeliverySubmitted as u8;
        Ok(())
    }

    pub fn release_to_seller(ctx: Context<ReleaseToSeller>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;
        require!(
            escrow.state == EscrowState::Funded as u8 || escrow.state == EscrowState::DeliverySubmitted as u8,
            EscrowError::InvalidState
        );
        require!(ctx.accounts.buyer.key() == escrow.buyer, EscrowError::Unauthorized);

        let amount = escrow.amount;
        let fee_bps = escrow.fee_bps as u64;
        let fee = amount.checked_mul(fee_bps).and_then(|v| v.checked_div(10_000)).unwrap_or(0);
        let seller_amount = amount.checked_sub(fee).ok_or(EscrowError::MathOverflow)?;

        let escrow_key = escrow.key();
        let seeds: &[&[u8]] = &[
            b"vault_authority",
            escrow_key.as_ref(),
            &[escrow.vault_authority_bump],
        ];
        let signer_seeds = &[seeds];

        if fee > 0 {
            let cpi_accounts_fee = TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.usdc_mint.to_account_info(),
                to: ctx.accounts.fee_token_account.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            };
            token::transfer_checked(
                CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts_fee, signer_seeds),
                fee,
                ctx.accounts.usdc_mint.decimals,
            )?;
        }

        let cpi_accounts_seller = TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.usdc_mint.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        token::transfer_checked(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts_seller, signer_seeds),
            seller_amount,
            ctx.accounts.usdc_mint.decimals,
        )?;

        let escrow = &mut ctx.accounts.escrow;
        escrow.state = EscrowState::Released as u8;
        escrow.settled_at_unix = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn open_dispute(ctx: Context<OpenDispute>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        require!(
            escrow.state == EscrowState::Funded as u8 || escrow.state == EscrowState::DeliverySubmitted as u8,
            EscrowError::InvalidState
        );
        require!(ctx.accounts.buyer.key() == escrow.buyer, EscrowError::Unauthorized);
        escrow.state = EscrowState::Disputed as u8;
        escrow.disputed_at_unix = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn resolve_dispute_refund_buyer(ctx: Context<ResolveDispute>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;
        require!(escrow.state == EscrowState::Disputed as u8, EscrowError::InvalidState);
        require!(
            ctx.accounts.arbitration_authority.key() == ctx.accounts.config.arbitration_authority,
            EscrowError::Unauthorized
        );

        let escrow_key = escrow.key();
        let seeds: &[&[u8]] = &[
            b"vault_authority",
            escrow_key.as_ref(),
            &[escrow.vault_authority_bump],
        ];
        let signer_seeds = &[seeds];

        let cpi_accounts = TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.usdc_mint.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        token::transfer_checked(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer_seeds),
            escrow.amount,
            ctx.accounts.usdc_mint.decimals,
        )?;

        let escrow = &mut ctx.accounts.escrow;
        escrow.state = EscrowState::Refunded as u8;
        escrow.settled_at_unix = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn resolve_dispute_pay_seller(ctx: Context<ResolveDisputePaySeller>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;
        require!(escrow.state == EscrowState::Disputed as u8, EscrowError::InvalidState);
        require!(
            ctx.accounts.arbitration_authority.key() == ctx.accounts.config.arbitration_authority,
            EscrowError::Unauthorized
        );

        let amount = escrow.amount;
        let fee_bps = escrow.fee_bps as u64;
        let fee = amount.checked_mul(fee_bps).and_then(|v| v.checked_div(10_000)).unwrap_or(0);
        let seller_amount = amount.checked_sub(fee).ok_or(EscrowError::MathOverflow)?;

        let escrow_key = escrow.key();
        let seeds: &[&[u8]] = &[
            b"vault_authority",
            escrow_key.as_ref(),
            &[escrow.vault_authority_bump],
        ];
        let signer_seeds = &[seeds];

        if fee > 0 {
            let cpi_accounts_fee = TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.usdc_mint.to_account_info(),
                to: ctx.accounts.fee_token_account.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            };
            token::transfer_checked(
                CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts_fee, signer_seeds),
                fee,
                ctx.accounts.usdc_mint.decimals,
            )?;
        }

        let cpi_accounts_seller = TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.usdc_mint.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        token::transfer_checked(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts_seller, signer_seeds),
            seller_amount,
            ctx.accounts.usdc_mint.decimals,
        )?;

        let escrow = &mut ctx.accounts.escrow;
        escrow.state = EscrowState::ResolvedToSeller as u8;
        escrow.settled_at_unix = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn cancel_unfunded(ctx: Context<CancelUnfunded>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;
        require!(escrow.state == EscrowState::Created as u8, EscrowError::InvalidState);
        require!(
            ctx.accounts.authority.key() == escrow.buyer || ctx.accounts.authority.key() == escrow.seller,
            EscrowError::Unauthorized
        );
        let escrow = &mut ctx.accounts.escrow;
        escrow.state = EscrowState::Cancelled as u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(escrow_id: [u8; 32])]
pub struct CreateEscrow<'info> {
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = buyer,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", escrow_id.as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// CHECK: PDA authority for vault
    #[account(
        seeds = [b"vault_authority", escrow.key().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = buyer,
        token::mint = usdc_mint,
        token::authority = vault_authority
    )]
    pub vault: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct FundEscrow<'info> {
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"escrow", escrow.escrow_id.as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,
    pub buyer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct MarkDelivery<'info> {
    #[account(
        mut,
        seeds = [b"escrow", escrow.escrow_id.as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub seller: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReleaseToSeller<'info> {
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"escrow", escrow.escrow_id.as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// CHECK: PDA
    #[account(
        seeds = [b"vault_authority", escrow.key().as_ref()],
        bump = escrow.vault_authority_bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub fee_token_account: Account<'info, TokenAccount>,

    pub buyer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct OpenDispute<'info> {
    #[account(
        mut,
        seeds = [b"escrow", escrow.escrow_id.as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub buyer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResolveDispute<'info> {
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"escrow", escrow.escrow_id.as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// CHECK: PDA
    #[account(
        seeds = [b"vault_authority", escrow.key().as_ref()],
        bump = escrow.vault_authority_bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,

    pub arbitration_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ResolveDisputePaySeller<'info> {
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"escrow", escrow.escrow_id.as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// CHECK: PDA
    #[account(
        seeds = [b"vault_authority", escrow.key().as_ref()],
        bump = escrow.vault_authority_bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub fee_token_account: Account<'info, TokenAccount>,

    pub arbitration_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CancelUnfunded<'info> {
    #[account(
        mut,
        seeds = [b"escrow", escrow.escrow_id.as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub usdc_mint: Pubkey,
    pub fee_bps: u16,
    pub fee_recipient: Pubkey,
    pub arbitration_authority: Pubkey,
    pub paused: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub version: u8,
    pub escrow_id: [u8; 32],
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub vault: Pubkey,
    pub amount: u64,
    pub fee_bps: u16,
    pub fee_recipient: Pubkey,
    pub agreement_hash: [u8; 32],
    pub deadline_unix: i64,
    pub state: u8,
    pub bump: u8,
    pub vault_authority_bump: u8,
    pub funded_at_unix: i64,
    pub disputed_at_unix: i64,
    pub settled_at_unix: i64,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EscrowState {
    Created = 0,
    Funded = 1,
    DeliverySubmitted = 2,
    Disputed = 3,
    Released = 4,
    ResolvedToSeller = 5,
    Refunded = 6,
    Cancelled = 7,
}

#[error_code]
pub enum EscrowError {
    #[msg("Fee exceeds maximum allowed")]
    FeeTooHigh,
    #[msg("Escrow is paused")]
    Paused,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Buyer and seller must differ")]
    BuyerSellerSame,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invalid deadline")]
    InvalidDeadline,
    #[msg("Invalid escrow state")]
    InvalidState,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Invalid vault")]
    InvalidVault,
}
