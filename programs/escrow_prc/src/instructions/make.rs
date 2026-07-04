use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use crate::state::EscrowState;

pub fn handler(ctx: Context<Make>, _seed: u64, receive_amount: u64, deposit_amount: u64) -> Result<()> {
    // store all escrow info in the state account
    let escrow = &mut ctx.accounts.escrow_state;
    escrow.maker = ctx.accounts.maker.key();
    escrow.mint_a = ctx.accounts.mint_a.key();
    escrow.mint_b = ctx.accounts.mint_b.key();
    escrow.receive_amount = receive_amount;
    escrow.bump = ctx.bumps.escrow_state;

    msg!("Escrow created by: {:?}", ctx.accounts.maker.key());
    msg!("Offering Token A: {:?}", ctx.accounts.mint_a.key());
    msg!("Wanting Token B: {:?}", ctx.accounts.mint_b.key());
    msg!("Deposit amount: {}", deposit_amount);
    msg!("Receive amount: {}", receive_amount);

    // CPI — transfer Token A from maker's ATA to vault
    // This is where CPI happens — our program calls SPL Token program
    let cpi_accounts = Transfer {
        from: ctx.accounts.maker_ata_a.to_account_info(),  // Alice's Token A account
        to: ctx.accounts.vault.to_account_info(),           // escrow vault
        authority: ctx.accounts.maker.to_account_info(),    // Alice authorizes
    };
    let cpi_ctx = CpiContext::new(
    ctx.accounts.token_program.key(),  // ✅ .key() not .to_account_info()
    cpi_accounts,
    );
    token::transfer(cpi_ctx, deposit_amount)?;

    msg!("Transferred {} Token A to vault", deposit_amount);
    Ok(())
}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    // Alice — the escrow maker
    #[account(mut)]
    pub maker: Signer<'info>,

    // Token A mint — the token Alice is offering
    pub mint_a: Account<'info, Mint>,

    // Token B mint — the token Alice wants
    pub mint_b: Account<'info, Mint>,

    // Alice's Token A account — tokens come FROM here
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: Account<'info, TokenAccount>,

    // Escrow state PDA — stores all escrow information
    #[account(
        init,
        payer = maker,
        space = 8 + EscrowState::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow_state: Account<'info, EscrowState>,

    // Vault — ATA owned by the escrow PDA — holds Token A
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow_state,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}