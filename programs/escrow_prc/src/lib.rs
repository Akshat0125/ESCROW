use anchor_lang::prelude::*;


declare_id!("3EXJ5DHYdqNungWGWR2S9wNMMucCr2pRVjPitfLFDfTi");

#[program]
pub mod escrow_prc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
