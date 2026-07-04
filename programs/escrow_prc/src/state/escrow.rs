use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowState {
    // who created the escrow (Alice)
    pub maker: Pubkey,

    // which token Alice is offering (Token A mint)
    pub mint_a: Pubkey,

    // which token Alice wants in return (Token B mint)
    pub mint_b: Pubkey,

    // how much Token B Alice wants
    pub receive_amount: u64,

    // bump seed for the escrow PDA
    pub bump: u8,
}