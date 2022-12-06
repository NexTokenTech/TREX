/// Provides a middleware interface for calling storage across pallets
pub trait TeeStorageInterface{
    /// Return value generic
    type Value;
    /// Account generic
    type AccountId;
    /// Determine whether this enclave account is included
    fn contains_account(sender:&Self::AccountId) -> Self::Value;
}