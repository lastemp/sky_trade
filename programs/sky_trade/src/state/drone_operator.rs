use anchor_lang::prelude::*;

/* #[derive(Default, Debug, AnchorSerialize, AnchorDeserialize, Copy, Clone, InitSpace)]
pub enum OperatorCategory {
    Individual,
    Company,
    #[default]
    None,
} */

/* 1: Individual,
2: Company */

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct DroneOperator {
    pub owner: Pubkey, // publickey of the drone operator
    #[max_len(50)]
    pub name: String, // name of drone operator
    //pub category: OperatorCategory, // category of drone operator
    pub category: u8, // category of drone operator
    #[max_len(3)]
    pub country: String, // country of drone operator
    pub active: bool, // status
    pub total_amount_available: u32, // total amount deposited
}
