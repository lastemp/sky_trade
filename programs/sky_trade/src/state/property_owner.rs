use anchor_lang::prelude::*;

/* #[derive(Default, Debug, AnchorSerialize, AnchorDeserialize, Copy, Clone, InitSpace)]
pub enum OwnerCategory {
    IndividualPropertyOwner,
    RealEstateCompany,
    CityMunicipality,
    #[default]
    None,
} */

/* 1: IndividualPropertyOwner,
2: RealEstateCompany,
3: CityMunicipality, */

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct PropertyOwner {
    pub owner: Pubkey, // publickey of the property owner
    #[max_len(50)]
    pub name: String, // name of property owner
    //pub category: OwnerCategory, // category of property owner
    pub category: u8, // category of property owner
    #[max_len(3)]
    pub country: String, // country of property owner
    pub active: bool, // status
    pub total_income_earned: u32, // total income earned
    pub total_income_available: u32, // total income earned less amount withdrawn
}
