//! ClaimAirspace instruction handler

use {
    crate::{
        error::SkyTradeError,
        state::{
            airspace::Airspace,
            property::{GpsCoordinates, Property},
            property_owner::PropertyOwner,
        },
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: ClaimAirspaceParams)]
pub struct ClaimAirspace<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + Property::INIT_SPACE,
        seeds = [b"property", owner.key().as_ref()],
        bump
    )]
    pub property: Account<'info, Property>,
    #[account(mut, has_one = owner)]
    pub property_owner: Account<'info, PropertyOwner>,
    // mut makes it changeble (mutable)
    /// CHECK: airspace account is initialized
    #[account(
        mut, constraint = airspace.is_initialized @ SkyTradeError::AccountNotInitialized
    )]
    pub airspace: Account<'info, Airspace>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ClaimAirspaceParams {
    name: String,                         // name of property
    country: String,                      // country where property is located
    property_coordinates: GpsCoordinates, // coordinates of the property
    cubic_feet: u32,                      // cubic feet volume of the property
}

// name length
const NAME_LENGTH: usize = 50;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn claim_airspace(ctx: Context<ClaimAirspace>, params: &ClaimAirspaceParams) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.name.as_bytes().len() > 0 && params.name.as_bytes().len() <= NAME_LENGTH {
    } else {
        return Err(SkyTradeError::InvalidNameLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(SkyTradeError::InvalidCountryLength.into());
    }

    if params.property_coordinates.latitude.as_bytes().len() == 0
        || params.property_coordinates.longitude.as_bytes().len() == 0
    {
        return Err(SkyTradeError::InvalidGpsCoordinates.into());
    }

    if params.cubic_feet == 0 {
        return Err(SkyTradeError::InvalidCubicFeet.into());
    }

    // property coordinates
    let latitude = params
        .property_coordinates
        .latitude
        .replace(" ", "")
        .to_string();
    let longitude = params
        .property_coordinates
        .longitude
        .replace(" ", "")
        .to_string();
    let is_valid_latitude = latitude.trim().parse::<f32>().is_ok();
    let is_valid_longitude = longitude.trim().parse::<f32>().is_ok();

    if !is_valid_latitude || !is_valid_longitude {
        return Err(SkyTradeError::InvalidGpsCoordinates.into());
    }

    let property = &mut ctx.accounts.property;
    let airspace = &mut ctx.accounts.airspace;

    // * - means dereferencing
    property.owner = *ctx.accounts.owner.key;
    property.name = params.name.to_string();
    property.country = params.country.to_string();
    property.property_coordinates.latitude = latitude.to_string();
    property.property_coordinates.longitude = longitude.to_string();
    property.cubic_feet = params.cubic_feet;
    property.is_claimed = true;

    //airspace
    let _coordinates = GpsCoordinates {
        latitude,
        longitude,
    };
    airspace.property_coordinates.push(_coordinates);

    Ok(())
}
