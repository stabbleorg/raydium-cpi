use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SwapSingle<'info> {
    #[account(signer)]
    pub payer: AccountInfo<'info>,
    pub amm_config: AccountInfo<'info>,
    #[account(mut)]
    pub pool_state: AccountInfo<'info>,
    #[account(mut)]
    pub input_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub output_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub input_vault: AccountInfo<'info>,
    #[account(mut)]
    pub output_vault: AccountInfo<'info>,
    #[account(mut)]
    pub observation_state: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub tick_array: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SwapSingleV2<'info> {
    #[account(signer)]
    pub payer: AccountInfo<'info>,
    pub amm_config: AccountInfo<'info>,
    #[account(mut)]
    pub pool_state: AccountInfo<'info>,
    #[account(mut)]
    pub input_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub output_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub input_vault: AccountInfo<'info>,
    #[account(mut)]
    pub output_vault: AccountInfo<'info>,
    #[account(mut)]
    pub observation_state: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub token_program_2022: AccountInfo<'info>,
    pub memo_program: AccountInfo<'info>,
    pub input_vault_mint: AccountInfo<'info>,
    pub output_vault_mint: AccountInfo<'info>,
    // remaining accounts
    // tickarray_bitmap_extension: must add account if need regardless the sequence
    // tick_array_account_1
    // tick_array_account_2
    // tick_array_account_...
}

#[derive(Accounts)]
pub struct SwapRouterBaseIn<'info> {
    #[account(signer)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub input_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub input_token_mint: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub token_program_2022: AccountInfo<'info>,
    pub memo_program: AccountInfo<'info>,
}
