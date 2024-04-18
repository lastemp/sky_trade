//! sky_trade program entrypoint

pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("FvLyA2w9BMTFDVSeNgHnVtZgRsDQ8srYf81JxSS95xrm");

#[program]
pub mod sky_trade {
    use super::*;

    // admin instructions
    pub fn init(ctx: Context<Init>, params: InitParams) -> Result<()> {
        instructions::init(ctx, &params)
    }

    // public instructions
    pub fn register_property_owner(
        ctx: Context<RegisterPropertyOwner>,
        params: RegisterPropertyOwnerParams,
    ) -> Result<()> {
        instructions::register_property_owner(ctx, &params)
    }

    pub fn register_drone_operator(
        ctx: Context<RegisterDroneOperator>,
        params: RegisterDroneOperatorParams,
    ) -> Result<()> {
        instructions::register_drone_operator(ctx, &params)
    }

    pub fn claim_airspace(ctx: Context<ClaimAirspace>, params: ClaimAirspaceParams) -> Result<()> {
        instructions::claim_airspace(ctx, &params)
    }

    pub fn rent_airspace(ctx: Context<RentAirspace>, params: RentAirspaceParams) -> Result<()> {
        instructions::rent_airspace(ctx, &params)
    }

    pub fn buy_sell_airspace(
        ctx: Context<BuySellAirspace>,
        params: BuySellAirspaceParams,
    ) -> Result<()> {
        instructions::buy_sell_airspace(ctx, &params)
    }
}
