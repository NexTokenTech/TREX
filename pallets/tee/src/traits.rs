pub trait TeeStorageInterface{
    type Value;
    type AccountId;
    fn contains_account(sender:&Self::AccountId) -> Self::Value;
}