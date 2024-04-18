use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Configs {
    pub price_per_cubic_foot: u32,        // price per cubic foot
    pub buying_price_per_cubic_foot: u32, // buying price per cubic foot
    pub total_income_earned: u32,         // total income earned from airspace
    pub total_income_available: u32,      // total income earned less amount withdrawn
    pub is_initialized: bool,
}
