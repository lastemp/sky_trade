//! RegisterPropertyOwner instruction handler

use {
    crate::{
        error::SkyTradeError,
        state::property_owner::{OwnerCategory, PropertyOwner},
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: RegisterPropertyOwnerParams)]
pub struct RegisterPropertyOwner<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + PropertyOwner::INIT_SPACE,
        seeds = [b"property-owner", owner.key().as_ref()],
        bump
    )]
    pub property_owner: Account<'info, PropertyOwner>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterPropertyOwnerParams {
    name: String,            // name of property owner
    category: OwnerCategory, // category of property owner
    country: String,         // country of property owner
}

// name length
const NAME_LENGTH: usize = 50;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn register_property_owner(
    ctx: Context<RegisterPropertyOwner>,
    params: &RegisterPropertyOwnerParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.name.as_bytes().len() > 0 && params.name.as_bytes().len() <= NAME_LENGTH {
    } else {
        return Err(SkyTradeError::InvalidNameLength.into());
    }

    let is_valid_category = {
        match params.category {
            OwnerCategory::None => false,
            _ => true,
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

    let property_owner = &mut ctx.accounts.property_owner;

    // * - means dereferencing
    property_owner.owner = *ctx.accounts.owner.key;
    property_owner.name = params.name.to_string();
    property_owner.category = params.category;
    property_owner.country = params.country.to_string();
    property_owner.active = true;

    Ok(())
}
