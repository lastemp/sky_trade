// admin instructions
pub mod init;

// public instructions
pub mod buy_sell_airspace;
pub mod claim_airspace;
pub mod register_drone_operator;
pub mod register_property_owner;
pub mod rent_airspace;

// bring everything in scope
pub use {
    buy_sell_airspace::*, claim_airspace::*, init::*, register_drone_operator::*,
    register_property_owner::*, rent_airspace::*,
};
