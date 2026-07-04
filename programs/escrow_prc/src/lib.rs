use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

// re-export structs at crate root — this is what #[program] macro needs
pub use instructions::make::*;
pub use instructions::take::*;
pub use instructions::refund::*;

declare_id!("3EXJ5DHYdqNungWGWR2S9wNMMucCr2pRVjPitfLFDfTi");

#[program]
pub mod escrow_prc {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        receive_amount: u64,
        deposit_amount: u64,
    ) -> Result<()> {
        instructions::make::handler(ctx, seed, receive_amount, deposit_amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        instructions::take::handler(ctx)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::handler(ctx)
    }
}