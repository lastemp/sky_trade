//! BuySellAirspace instruction handler

use {
    crate::{
        error::SkyTradeError,
        state::{
            airspace::Airspace, configs::Configs, deposit_base::DepositBase, property::Property,
            property_owner::PropertyOwner,
        },
    },
    anchor_lang::{prelude::*, system_program},
};

#[derive(Accounts)]
#[instruction(params: BuySellAirspaceParams)]
pub struct BuySellAirspace<'info> {
    #[account(mut, constraint = buyer.active @ SkyTradeError::InvalidPropertyOwnerStatus)]
    pub buyer: Account<'info, PropertyOwner>,
    #[account(mut, constraint = seller.active @ SkyTradeError::InvalidPropertyOwnerStatus)]
    pub seller: Account<'info, PropertyOwner>,
    #[account(mut, constraint = property.owner == seller.owner @ SkyTradeError::PersonDoesNotOwnAirspace)]
    pub property: Account<'info, Property>, // Enforce the check that property(airspace) must be owned by seller
    // mut makes it changeble (mutable)
    /// CHECK: airspace account is initialized
    #[account(
        mut, constraint = airspace.is_initialized @ SkyTradeError::AccountNotInitialized
    )]
    pub airspace: Account<'info, Airspace>,
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
pub struct BuySellAirspaceParams {
    pub cubic_feet: u32, //cubic feet volume
}

pub fn buy_sell_airspace(
    ctx: Context<BuySellAirspace>,
    params: &BuySellAirspaceParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.cubic_feet == 0 {
        return Err(SkyTradeError::InvalidCubicFeet.into());
    }

    let deposit_auth = &ctx.accounts.owner;
    let sys_program = &ctx.accounts.system_program;

    let airspace = &mut ctx.accounts.airspace;
    let configs = &mut ctx.accounts.configs;
    let buyer = &mut ctx.accounts.buyer;
    let seller = &mut ctx.accounts.seller;
    let property = &mut ctx.accounts.property;

    let property_cubic_feet = property.cubic_feet; //cubic feet volume

    // Lets check if the property_cubic_feet matches
    // cubic_feet meant to be bought
    if params.cubic_feet != property_cubic_feet {
        return Err(SkyTradeError::InvalidCubicFeet.into());
    }

    /* // Lets check if the seller owns airspace
    let mut is_person_owns_airspace = false;
    for property in airspace.properties.iter() {
        if property.owner == seller.owner {
            is_person_owns_airspace = true;
            break;
        }
    }

    if !is_person_owns_airspace {
        return Err(SkyTradeError::PersonDoesNotOwnAirspace.into());
    } */

    // Lets check if the seller owns airspace
    let mut is_person_owns_airspace = false;
    if property.owner == seller.owner {
        is_person_owns_airspace = true;
    }

    if !is_person_owns_airspace {
        return Err(SkyTradeError::PersonDoesNotOwnAirspace.into());
    }

    // buying price per cubic foot. Used to calculate total
    // amount that the buyer will pay
    let buying_price_per_cubic_foot: u32 = configs.buying_price_per_cubic_foot;
    let total_income_earned_configs = configs.total_income_earned;
    let total_income_earned = seller.total_income_earned;
    let total_income_available = seller.total_income_available;
    let cubic_feet = params.cubic_feet;

    // Calculate the amount to be paid by property owner
    let cubic_feet_amount = cubic_feet
        .checked_mul(buying_price_per_cubic_foot)
        .ok_or(SkyTradeError::InvalidArithmeticOperation)?;

    // Lets increment this account's total_income_earned with amount to be paid
    configs.total_income_earned = total_income_earned_configs
        .checked_add(cubic_feet_amount)
        .ok_or(SkyTradeError::InvalidArithmeticOperation)?;

    // Lets increment this account's total_income_earned with amount to be paid
    seller.total_income_earned = total_income_earned
        .checked_add(cubic_feet_amount)
        .ok_or(SkyTradeError::InvalidArithmeticOperation)?;

    // Lets increment this account's total_income_available with amount to be paid
    seller.total_income_available = total_income_available
        .checked_add(cubic_feet_amount)
        .ok_or(SkyTradeError::InvalidArithmeticOperation)?;

    // Lets allocate the airspace to the new buyer
    property.owner = *ctx.accounts.owner.key;

    let lamports: u64 = 1_000_000_000; // 1 SOL = 1,000,000,000 lamports
    let _amount = (cubic_feet_amount as u64)
        .checked_mul(lamports)
        .ok_or(SkyTradeError::InvalidArithmeticOperation)?;

    // transfer sol from buyer to treasury vault
    let cpi_accounts = system_program::Transfer {
        from: deposit_auth.to_account_info(),
        to: ctx.accounts.admin_sol_vault.to_account_info(),
    };

    let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

    system_program::transfer(cpi, _amount.into())?;

    Ok(())
}
