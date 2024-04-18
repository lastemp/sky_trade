use anchor_lang::prelude::*;

#[derive(Default, Debug, AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct GpsCoordinates {
    #[max_len(10)]
    pub latitude: String, // latitude
    #[max_len(10)]
    pub longitude: String, // longitude
}

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Property {
    pub owner: Pubkey, // publickey of the property owner
    #[max_len(50)]
    pub name: String, // name of property
    #[max_len(3)]
    pub country: String, // country where property is located
    pub property_coordinates: GpsCoordinates, // coordinates of the property
    pub cubic_feet: u32, // cubic feet volume of the property
    pub is_claimed: bool, // status
}
