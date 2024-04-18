use anchor_lang::prelude::*;

#[derive(Default, Debug, AnchorSerialize, AnchorDeserialize, Copy, Clone, InitSpace)]
pub enum OwnerCategory {
    IndividualPropertyOwner,
    RealEstateCompany,
    CityMunicipality,
    #[default]
    None,
}

#[derive(Default, Debug, AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct GpsCoordinates {
    #[max_len(10)]
    pub latitude: String, // latitude
    #[max_len(10)]
    pub longitude: String, // longitude
}

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct PropertyOwner {
    pub owner: Pubkey, // publickey of the property owner
    #[max_len(50)]
    pub name: String, // name of property owner
    pub category: OwnerCategory, // category of property owner
    #[max_len(3)]
    pub country: String, // country of property owner
    pub active: bool,  // status
    //pub property_coordinates: GpsCoordinates, // coordinates of the property
    pub total_income_earned: u32,    // total income earned
    pub total_income_available: u32, // total income earned less amount withdrawn
}
