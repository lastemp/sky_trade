use {crate::state::property::GpsCoordinates, anchor_lang::prelude::*};

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Airspace {
    #[max_len(10)] //This value is just for test purposes
    pub property_coordinates: Vec<GpsCoordinates>, // coordinates of the property
    pub is_initialized: bool,
}
