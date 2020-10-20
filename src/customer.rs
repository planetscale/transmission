use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::seq::SliceRandom;
use rand::Rng;
use std::sync::atomic::Ordering;
use super::{GLOBAL_COUNTER,CLIENT_POOL};
use mysql::prelude::*;
use crate::repository::CustomerRepository;
use std::thread::sleep;
use std::time::Duration;

const CONNECTIVITY_RETRY_SECONDS: u64 = 1;

pub struct Customer {
    pub id: u32,
    pub name: String,
    pub national_id: String,
    pub country: String,
}

impl Customer {
    pub fn random() -> Customer {
        Customer {
            id: gen_id(),
            name: gen_name(),
            national_id: gen_national_id(),
            country: gen_country(),
        }
    }

    // Escapes apostrophes in MySQL compatible way before returning name.
    pub fn sql_name(&self) -> String {
        escape_apostrophe(&self.name)
    }
}

pub struct Customers {
    pub conn: mysql::PooledConn,
}

impl Customers {
    pub fn new() -> Customers {
        Customers {
            conn: CLIENT_POOL.get_conn().unwrap(),
        }
    }
}

impl CustomerRepository for Customers {
    fn insert(&mut self, customer: &Customer) -> Result<u32, mysql::Error> {
        match self.conn.query_drop(format!(
            r"INSERT INTO customer (id, full_name, national_id, country) VALUES ('{}', '{}', '{}', '{}')",
            customer.id, customer.sql_name(), customer.national_id, customer.country,
        )) {
            Ok(_) => Ok(customer.id),
            Err(e) => Err(e),
        }
    }
}

pub struct CustomersRetryWrapper {
    pub customers: Customers
}

impl CustomersRetryWrapper {
    pub fn new(customers: Customers) -> CustomersRetryWrapper{
        CustomersRetryWrapper {
            customers,
        }
    }
}

impl CustomerRepository for CustomersRetryWrapper {
    fn insert(&mut self, customer: &Customer) -> Result<u32, mysql::Error> {
        for _ in 0..10 {
            match self.customers.insert(customer) {
                Ok(id) => return Ok(id),
                Err(err) => {
                    if err.is_connectivity_error() {
                        println!("Failed to connect to mysql endpoint. Retrying in {} seconds.", CONNECTIVITY_RETRY_SECONDS)
                    } else {
                        println!("Non-connectivity error: {}; Retrying in {} seconds.", err, CONNECTIVITY_RETRY_SECONDS)
                    }
                },
            }
            sleep(Duration::from_secs(CONNECTIVITY_RETRY_SECONDS))
        }
        self.customers.insert(customer)
    }
}

fn gen_name() -> String {
    Name(EN).fake()
}

fn gen_id() -> u32 {
    GLOBAL_COUNTER.fetch_add(1,Ordering::SeqCst)
}

fn gen_country() -> String {
    let vs = vec!["United States","Canada","France","Germany","China","Japan","India","Indonesia"];
    vs.choose(&mut rand::thread_rng()).unwrap().to_string()
}

fn gen_national_id() -> String {
    let nums: Vec<String> = (0..3).map(|_| {
        gen_three_digits().to_string()
    }).collect();
    nums.join("-")
}

fn gen_three_digits() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(100, 1000)
}

fn escape_apostrophe(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        if c == '\'' {
            output.push('\'')
        }
        output.push(c)
    }
    output
}
