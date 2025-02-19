#[cfg(test)]
mod tests;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use anchor_lang::solana_program::program::invoke_signed;

declare_id!("HSP1yi2aHiKBpYVKv6QkURWSjMhr5WEz3xYdZktBFGVd");

#[program]
pub mod bigballs_token {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        decimals: u8,
        marketing_wallet: Pubkey,
    ) -> Result<()> {
        // Validate marketing wallet
        require!(
            marketing_wallet != Pubkey::default(),
            BigBallsError::InvalidMarketingWallet
        );

        let config = &mut ctx.accounts.config;
        
        // Basic token configuration
        config.name = name;
        config.symbol = symbol;
        config.decimals = decimals;
        config.total_supply = 1_000_000_000 * 10u64.pow(decimals as u32);  // 1 billion tokens
        config.authority = ctx.accounts.authority.key();
        config.marketing_wallet = marketing_wallet;
        
        // Trading parameters
        config.trading_enabled = false;
        config.max_transaction_amount = config.total_supply / 1000; // 0.1% of total supply
        config.max_wallet_amount = config.total_supply / 100;      // 1% of total supply
        
        // Fee configuration (in basis points)
        config.reflection_fee_bp = 200;     // 2%
        config.marketing_fee_bp = 150;      // 1.5%
        config.burn_fee_bp = 100;           // 1%
        config.dev_fee_bp = 50;             // 0.5%
        
        // Cooldown periods (in seconds)
        config.buy_cooldown = 300;          // 5 minutes
        config.sell_cooldown = 1800;        // 30 minutes
        config.transaction_cooldown = 60;    // 1 minute
        
        // Initialize the mint
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            decimals,
            &config.authority,
            Some(&config.authority),
        )?;

        emit!(TokenInitialized {
            mint: ctx.accounts.mint.key(),
            authority: config.authority,
            total_supply: config.total_supply,
            decimals,
            marketing_wallet,
        });

        Ok(())
    }

    pub fn transfer(
        ctx: Context<Transfer>,
        amount: u64,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let clock = Clock::get()?;

        // Check trading enabled
        require!(config.trading_enabled, BigBallsError::TradingNotEnabled);

        // Check transfer limits
        require!(
            amount <= config.max_transaction_amount,
            BigBallsError::ExceedsMaxTransaction
        );

        // Check cooldown periods
        let holder_state = &mut ctx.accounts.holder_state;
        require!(
            clock.unix_timestamp >= holder_state.last_transaction + config.transaction_cooldown,
            BigBallsError::CooldownNotElapsed
        );

        // Calculate fees
        let total_fee_bp = config.reflection_fee_bp + config.marketing_fee_bp + 
                          config.burn_fee_bp + config.dev_fee_bp;
        let total_fee = amount * total_fee_bp as u64 / 10000;
        
        let reflection_amount = amount * config.reflection_fee_bp as u64 / 10000;
        let marketing_amount = amount * config.marketing_fee_bp as u64 / 10000;
        let burn_amount = amount * config.burn_fee_bp as u64 / 10000;
        let dev_amount = amount * config.dev_fee_bp as u64 / 10000;

        // Transfer main amount minus fees
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            amount - total_fee,
        )?;

        // Transfer fees
        if marketing_amount > 0 {
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    token::Transfer {
                        from: ctx.accounts.from.to_account_info(),
                        to: ctx.accounts.marketing_wallet.to_account_info(),
                        authority: ctx.accounts.authority.to_account_info(),
                    },
                ),
                marketing_amount,
            )?;
        }

        // Update holder state
        holder_state.last_transaction = clock.unix_timestamp;
        holder_state.total_transactions += 1;
        holder_state.total_amount += amount;

        // Emit events
        emit!(TransferExecuted {
            from: ctx.accounts.from.key(),
            to: ctx.accounts.to.key(),
            amount: amount - total_fee,
            timestamp: clock.unix_timestamp,
        });

        emit!(FeesCollected {
            reflection_amount,
            marketing_amount,
            burn_amount,
            dev_amount,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    pub fn set_trading_enabled(
        ctx: Context<SetTrading>,
        enabled: bool
    ) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ctx.accounts.config.authority,
            BigBallsError::Unauthorized
        );
        ctx.accounts.config.trading_enabled = enabled;
        
        emit!(TradingStatusChanged {
            enabled,
            timestamp: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }

    pub fn update_fees(
        ctx: Context<UpdateFees>,
        reflection_fee_bp: u16,
        marketing_fee_bp: u16,
        burn_fee_bp: u16,
        dev_fee_bp: u16,
    ) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ctx.accounts.config.authority,
            BigBallsError::Unauthorized
        );

        // Validate total fees don't exceed reasonable limit (e.g., 10%)
        let total_fee_bp = reflection_fee_bp + marketing_fee_bp + burn_fee_bp + dev_fee_bp;
        require!(total_fee_bp <= 1000, BigBallsError::InvalidFeeConfiguration);

        let config = &mut ctx.accounts.config;
        config.reflection_fee_bp = reflection_fee_bp;
        config.marketing_fee_bp = marketing_fee_bp;
        config.burn_fee_bp = burn_fee_bp;
        config.dev_fee_bp = dev_fee_bp;

        emit!(FeesUpdated {
            reflection_fee_bp,
            marketing_fee_bp,
            burn_fee_bp,
            dev_fee_bp,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + TokenConfig::LEN)]
    pub config: Account<'info, TokenConfig>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub config: Account<'info, TokenConfig>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut)]
    pub marketing_wallet: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + HolderState::LEN,
        seeds = [b"holder_state", from.key().as_ref()],
        bump
    )]
    pub holder_state: Account<'info, HolderState>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SetTrading<'info> {
    #[account(mut)]
    pub config: Account<'info, TokenConfig>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateFees<'info> {
    #[account(mut)]
    pub config: Account<'info, TokenConfig>,
    pub authority: Signer<'info>,
}

#[account]
pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub trading_enabled: bool,
    pub authority: Pubkey,
    pub marketing_wallet: Pubkey,
    
    // Trading limits
    pub max_transaction_amount: u64,
    pub max_wallet_amount: u64,
    
    // Fee configuration (in basis points)
    pub reflection_fee_bp: u16,
    pub marketing_fee_bp: u16,
    pub burn_fee_bp: u16,
    pub dev_fee_bp: u16,
    
    // Cooldown periods (in seconds)
    pub buy_cooldown: i64,
    pub sell_cooldown: i64,
    pub transaction_cooldown: i64,
}

#[account]
pub struct HolderState {
    pub last_transaction: i64,
    pub total_transactions: u64,
    pub total_amount: u64,
    pub last_reflection_claim: i64,
}

impl TokenConfig {
    pub const LEN: usize = 32 + // name
                          32 + // symbol
                          1 +  // decimals
                          8 +  // total_supply
                          1 +  // trading_enabled
                          32 + // authority
                          32 + // marketing_wallet
                          8 +  // max_transaction_amount
                          8 +  // max_wallet_amount
                          2 +  // reflection_fee_bp
                          2 +  // marketing_fee_bp
                          2 +  // burn_fee_bp
                          2 +  // dev_fee_bp
                          8 +  // buy_cooldown
                          8 +  // sell_cooldown
                          8;   // transaction_cooldown
}

impl HolderState {
    pub const LEN: usize = 8 +  // last_transaction
                          8 +  // total_transactions
                          8 +  // total_amount
                          8;   // last_reflection_claim
}

#[event]
pub struct TokenInitialized {
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub total_supply: u64,
    pub decimals: u8,
    pub marketing_wallet: Pubkey,
}

#[event]
pub struct TransferExecuted {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct FeesCollected {
    pub reflection_amount: u64,
    pub marketing_amount: u64,
    pub burn_amount: u64,
    pub dev_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct TradingStatusChanged {
    pub enabled: bool,
    pub timestamp: i64,
}

#[event]
pub struct FeesUpdated {
    pub reflection_fee_bp: u16,
    pub marketing_fee_bp: u16,
    pub burn_fee_bp: u16,
    pub dev_fee_bp: u16,
    pub timestamp: i64,
}

#[error_code]
pub enum BigBallsError {
    #[msg("Trading is not enabled")]
    TradingNotEnabled,
    #[msg("Transaction amount exceeds limit")]
    ExceedsMaxTransaction,
    #[msg("Wallet amount exceeds limit")]
    ExceedsMaxWallet,
    #[msg("Cooldown period not elapsed")]
    CooldownNotElapsed,
    #[msg("Invalid fee calculation")]
    InvalidFeeCalculation,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid marketing wallet address")]
    InvalidMarketingWallet,
    #[msg("Invalid fee configuration")]
    InvalidFeeConfiguration,
}
