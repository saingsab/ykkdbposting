// Use mode or craite for mssql connection
mod conn;
// start building SP
// DB
use serde_json;
use std::fs;


// For ENV
#[macro_use]
extern crate dotenv_codegen;
#[tokio::main]
async fn main() {

        // Calling from other mod;
        conn::conn().await;

    println!("init... calling env  ");
    println!("init... calling Configuration File  ");
    let path = "./storeconfiguration.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    println!("{}", res["address"]["city"]);
    println!("Phone Number: {}", res["phones"][0]);


}