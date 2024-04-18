//! RegisterDroneOperator instruction handler

use {
    crate::{error::SkyTradeError, state::drone_operator::DroneOperator},
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: RegisterDroneOperatorParams)]
pub struct RegisterDroneOperator<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + DroneOperator::INIT_SPACE,
        seeds = [b"drone-operator", owner.key().as_ref()],
        bump
    )]
    pub drone_operator: Account<'info, DroneOperator>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterDroneOperatorParams {
    name: String, // name of drone operator
    //category: OperatorCategory, // category of drone operator
    category: u8,    // category of drone operator
    country: String, // country of drone operator
}
// name length
const NAME_LENGTH: usize = 50;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn register_drone_operator(
    ctx: Context<RegisterDroneOperator>,
    params: &RegisterDroneOperatorParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.name.as_bytes().len() > 0 && params.name.as_bytes().len() <= NAME_LENGTH {
    } else {
        return Err(SkyTradeError::InvalidNameLength.into());
    }

    /* 1: Individual,
    2: Company */

    let is_valid_category = {
        match params.category {
            1 | 2 => true,
            _ => false,
        }
    };

    if !is_valid_category {
        return Err(SkyTradeError::InvalidCategory.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(SkyTradeError::InvalidCountryLength.into());
    }

    let drone_operator = &mut ctx.accounts.drone_operator;

    // * - means dereferencing
    drone_operator.owner = *ctx.accounts.owner.key;
    drone_operator.name = params.name.to_string();
    drone_operator.category = params.category;
    drone_operator.country = params.country.to_string();
    drone_operator.active = true;

    Ok(())
}
