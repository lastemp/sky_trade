use crate::state::property::Property;
use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Airspace {
    #[max_len(10)] //This value is just for test purposes
    pub properties: Vec<Property>, // property
    pub is_initialized: bool,
}
