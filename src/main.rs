extern crate odbc;
// Use this crate and set environment variable RUST_LOG=odbc to see ODBC warnings
extern crate env_logger;
#[macro_use]
extern crate dotenv_codegen;
use odbc::*;
use std::io;
use odbc_safe::AutocommitOn;
use std::error::Error;

use csv;
use serde::Deserialize;

mod csv_read_serde;
mod csv_write_serde;

#[derive(Debug, Deserialize)]
struct Customer {
    customer_guid: String,
    first_name: String,
    last_name: String,
    email: String,
    address: String,
}

fn main() {

    // env_logger::init();

    // match connect() {
    //     Ok(()) => println!("Success"),
    //     Err(diag) => println!("Error: {}", diag),
    // }

    // Try CSV
    println!("Hi it works");
    // read_and_write("rows.csv", "rows-copy.csv").unwrap();
    // read_from_file("ayoung.csv").unwrap();
        // If an error occurs print error
    if let Err(e) = csv_read_serde::read_from_file("./ayoung.csv") {
        eprintln!("{}", e);
    }
    if let Err(e) = csv_write_serde::write_to_file("./ayoung.csv") {
        eprintln!("{}", e);
    }
    // read_from_file
}
