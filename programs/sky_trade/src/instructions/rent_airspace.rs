//! RentAirspace instruction handler

use {
    crate::{
        error::SkyTradeError,
        state::{
            airspace::Airspace, configs::Configs, deposit_base::DepositBase,
            drone_operator::DroneOperator, property_owner::PropertyOwner,
        },
    },
    anchor_lang::{prelude::*, system_program},
};

#[derive(Accounts)]
#[instruction(params: RentAirspaceParams)]
pub struct RentAirspace<'info> {
    #[account(mut, has_one = owner, constraint = drone_operator.active @ SkyTradeError::InvalidDroneOperatorStatus)]
    pub drone_operator: Account<'info, DroneOperator>,
    // mut makes it changeble (mutable)
    /// CHECK: airspace account is initialized
    #[account(
        mut, constraint = airspace.is_initialized @ SkyTradeError::AccountNotInitialized
    )]
    pub airspace: Account<'info, Airspace>,
    #[account(mut, constraint = property_owner.active @ SkyTradeError::InvalidPropertyOwnerStatus)]
    pub property_owner: Account<'info, PropertyOwner>,
    #[account(mut)]
    pub configs: Account<'info, Configs>,
    //admin accs
    #[account(mut,
        constraint = admin_deposit_account.is_initialized @ SkyTradeError::AccountNotInitialized
    )]
    pub admin_deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"admin-auth", admin_deposit_account.key().as_ref()], bump = admin_deposit_account.admin_auth_bump)]
    /// CHECK: no need to check this.
    pub admin_pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"admin-sol-vault", admin_pda_auth.key().as_ref()], bump = admin_deposit_account.admin_sol_vault_bump.unwrap())]
    pub admin_sol_vault: SystemAccount<'info>,
    //admin accs
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RentAirspaceParams {
    pub cubic_feet: u32, //cubic feet volume to be rented by drone operator
}

pub fn rent_airspace(ctx: Context<RentAirspace>, params: &RentAirspaceParams) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.cubic_feet == 0 {
        return Err(SkyTradeError::InvalidCubicFeet.into());
    }

    let deposit_auth = &ctx.accounts.owner;
    let sys_program = &ctx.accounts.system_program;

    let airspace = &mut ctx.accounts.airspace;
    let drone_operator = &mut ctx.accounts.drone_operator;
    let configs = &mut ctx.accounts.configs;
    let property_owner = &mut ctx.accounts.property_owner;

    // Lets check if the property owner has claimed airspace
    let mut is_property_owner_claimed_airspace = false;
    for property in airspace.properties.iter() {
        if property.owner == property_owner.owner {
            is_property_owner_claimed_airspace = true;
            break;
        }
    }

    if !is_property_owner_claimed_airspace {
        return Err(SkyTradeError::PropertyOwnerNotClaimedAirspace.into());
    }

    let price_per_cubic_foot: u32 = configs.price_per_cubic_foot;
    let total_income_earned_configs = configs.total_income_earned;
    let total_income_earned = property_owner.total_income_earned;
    let total_income_available = property_owner.total_income_available;
    let cubic_feet = params.cubic_feet;

    // Calculate the amount to be paid by drone operator
    let cubic_feet_amount = cubic_feet
        .checked_mul(price_per_cubic_foot)
        .ok_or(SkyTradeError::InvalidArithmeticOperation)?;

    // Lets increment this account's total_income_earned with amount to be paid
    configs.total_income_earned = total_income_earned_configs
        .checked_add(cubic_feet_amount)
        .ok_or(SkyTradeError::InvalidArithmeticOperation)?;

    // Lets increment this account's total_income_earned with amount to be paid
    property_owner.total_income_earned = total_income_earned
        .checked_add(cubic_feet_amount)
        .ok_or(SkyTradeError::InvalidArithmeticOperation)?;

    // Lets increment this account's total_income_available with amount to be paid
    property_owner.total_income_available = total_income_available
        .checked_add(cubic_feet_amount)
        .ok_or(SkyTradeError::InvalidArithmeticOperation)?;

    let lamports: u64 = 1_000_000_000; // 1 SOL = 1,000,000,000 lamports
    let _amount = (cubic_feet_amount as u64)
        .checked_mul(lamports)
        .ok_or(SkyTradeError::InvalidArithmeticOperation)?;

    // transfer sol from drone operator to treasury vault
    let cpi_accounts = system_program::Transfer {
        from: deposit_auth.to_account_info(),
        to: ctx.accounts.admin_sol_vault.to_account_info(),
    };

    let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

    system_program::transfer(cpi, _amount.into())?;

    Ok(())
}
