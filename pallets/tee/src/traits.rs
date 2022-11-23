pub trait TeeStorageInterface{
    type Value;
    type AccountId;
    fn contain_account(sender:&Self::AccountId) -> Self::Value;
}