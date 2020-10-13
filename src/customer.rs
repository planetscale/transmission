use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::seq::SliceRandom;
use rand::Rng;
use std::sync::atomic::Ordering;
use super::{GLOBAL_COUNTER,CLIENT_POOL};
use mysql::prelude::*;


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

pub struct CustomerRepo {
    pub conn: mysql::PooledConn,
}

impl CustomerRepo {
    pub fn new() -> CustomerRepo {
        CustomerRepo {
            conn: CLIENT_POOL.get_conn().unwrap(),
        }
    }

    pub fn insert(&mut self, customer: &Customer) -> Result<(), mysql::Error> {
        match self.conn.query_drop(format!(
            r"INSERT INTO customer (id, full_name, national_id, country) VALUES ('{}', '{}', '{}', '{}')",
            customer.id, customer.sql_name(), customer.national_id, customer.country,
        )) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
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
