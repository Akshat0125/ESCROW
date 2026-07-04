use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, CloseAccount, Mint, Token, TokenAccount, Transfer},
};
use crate::state::EscrowState;

pub fn handler(ctx: Context<Refund>) -> Result<()> {
    let escrow = &ctx.accounts.escrow_state;
    let maker_key = escrow.maker;
    let bump = escrow.bump;
    let seed = ctx.accounts.escrow_state.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"escrow",
        maker_key.as_ref(),
        seed.as_ref(),
        &[bump],
    ]];

    msg!("Refunding escrow to maker: {:?}", ctx.accounts.maker.key());

    // CPI 1 — return Token A from vault back to Alice
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.maker_ata_a.to_account_info(),
        authority: ctx.accounts.escrow_state.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.key(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_ctx, ctx.accounts.vault.amount)?;
    msg!("Refunded {} Token A to maker", ctx.accounts.vault.amount);

    // CPI 2 — close vault and return rent to Alice
    let cpi_accounts = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        destination: ctx.accounts.maker.to_account_info(),
        authority: ctx.accounts.escrow_state.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.key(),
        cpi_accounts,
        signer_seeds,
    );
    token::close_account(cpi_ctx)?;
    msg!("Vault closed and refund complete");

    Ok(())
}

#[derive(Accounts)]
pub struct Refund<'info> {
    // Alice — only she can refund
    #[account(mut)]
    pub maker: Signer<'info>,

    pub mint_a: Account<'info, Mint>,

    // Alice's Token A account — refunded tokens go here
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: Account<'info, TokenAccount>,

    // Escrow state — verify maker owns it
    #[account(
        mut,
        has_one = maker,
        has_one = mint_a,
        seeds = [b"escrow", maker.key().as_ref(), escrow_state.key().as_ref()],
        bump = escrow_state.bump,
        close = maker  // close and return rent
    )]
    pub escrow_state: Account<'info, EscrowState>,

    // Vault holding Token A
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow_state,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}