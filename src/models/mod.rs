pub mod bw_account;
pub mod types;

pub trait Crud {
    type Error;
    fn create(&self, item: &Self) -> Result<(), Self::Error>;
    fn read(&self) -> Result<Vec<Self>, Self::Error>
    where
        Self: Sized;
    fn update(&self, item: &Self) -> Result<(), Self::Error>;
    fn delete(&self, item: &Self) -> Result<(), Self::Error>;
}
