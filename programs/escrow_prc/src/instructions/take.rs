use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, CloseAccount, Mint, Token, TokenAccount, Transfer},
};
use crate::state::EscrowState;

pub fn handler(ctx: Context<Take>) -> Result<()> {
    let escrow = &ctx.accounts.escrow_state;

    msg!("Taker: {:?}", ctx.accounts.taker.key());
    msg!("Completing escrow swap");

    // CPI 1 — Bob sends Token B to Alice
    // Bob's Token B → Alice's Token B ATA
    let cpi_accounts = Transfer {
        from: ctx.accounts.taker_ata_b.to_account_info(),   // Bob's Token B
        to: ctx.accounts.maker_ata_b.to_account_info(),     // Alice's Token B
        authority: ctx.accounts.taker.to_account_info(),    // Bob authorizes
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.key(),
        cpi_accounts,
    );
    token::transfer(cpi_ctx, escrow.receive_amount)?;
    msg!("Sent {} Token B to maker", escrow.receive_amount);

    // CPI 2 — vault releases Token A to Bob
    // Vault → Bob's Token A ATA
    // This uses PDA signing — the escrow PDA signs on behalf of the vault
    let maker_key = escrow.maker;
    let bump = escrow.bump;
    let seed = ctx.accounts.escrow_state.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"escrow",
        maker_key.as_ref(),
        seed.as_ref(),
        &[bump],
    ]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),          // vault releases
        to: ctx.accounts.taker_ata_a.to_account_info(),      // Bob receives Token A
        authority: ctx.accounts.escrow_state.to_account_info(), // PDA signs
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.key(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_ctx, ctx.accounts.vault.amount)?;
    msg!("Released {} Token A to taker", ctx.accounts.vault.amount);

    // CPI 3 — close the vault and return rent to Alice
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
    msg!("Vault closed, rent returned to maker");
    msg!("Swap completed successfully");

    Ok(())
}

#[derive(Accounts)]
pub struct Take<'info> {
    // Bob — the escrow taker
    #[account(mut)]
    pub taker: Signer<'info>,

    // Alice — the escrow maker (receives Token B)
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    // Token A mint
    pub mint_a: Account<'info, Mint>,

    // Token B mint
    pub mint_b: Account<'info, Mint>,

    // Bob's Token A account — receives Token A from vault
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: Account<'info, TokenAccount>,

    // Bob's Token B account — Token B comes FROM here
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: Account<'info, TokenAccount>,

    // Alice's Token B account — receives Token B from Bob
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: Account<'info, TokenAccount>,

    // Escrow state PDA — read escrow info
    #[account(
        mut,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow_state.key().as_ref()],
        bump = escrow_state.bump,
        close = maker  // close escrow and return rent to Alice when done
    )]
    pub escrow_state: Account<'info, EscrowState>,

    // Vault — holds Token A
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