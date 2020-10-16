use crate::customer::Customer;

pub trait CustomerRepository {
    fn insert(&mut self, customer: &Customer) -> Result<u32, mysql::Error>;
}