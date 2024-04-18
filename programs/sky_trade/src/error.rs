use anchor_lang::prelude::*;

#[error_code]
pub enum SkyTradeError {
    // property owner/drone operator
    #[msg("Invalid name length")]
    InvalidNameLength,
    #[msg("Invalid country length")]
    InvalidCountryLength,
    #[msg("Invalid Category")]
    InvalidCategory,
    #[msg("Property owner has no active status.")]
    InvalidPropertyOwnerStatus,
    #[msg("Drone operator has no active status.")]
    InvalidDroneOperatorStatus,
    #[msg("Property owner has not claimed airspace.")]
    PropertyOwnerNotClaimedAirspace,
    #[msg("Person does not own airspace.")]
    PersonDoesNotOwnAirspace,

    // property
    #[msg("Gps coordinates has invalid values.")]
    InvalidGpsCoordinates,
    #[msg("Invalid cubic feet.")]
    InvalidCubicFeet,

    // configs
    #[msg("Price per cubic foot has invalid value.")]
    InvalidPricePerCubicFoot,

    // amount
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Insufficient funds.")]
    InsufficientFunds,
    #[msg("Invalid withdrawal amount.")]
    InvalidWithdrawalAmount,
    #[msg("Arithmetic operation failed.")]
    InvalidArithmeticOperation,

    //
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}
