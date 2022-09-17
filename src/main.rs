extern crate odbc;
// Use this crate and set environment variable RUST_LOG=odbc to see ODBC warnings
extern crate env_logger;
#[macro_use]
extern crate dotenv_codegen;
use odbc::*;
use std::io;
use odbc_safe::AutocommitOn;

fn main() {

    env_logger::init();

    match connect() {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {}", diag),
    }
}

fn connect() -> std::result::Result<(), DiagnosticRecord> {

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    let mut buffer = String::new();

    // Loading ENV
    let srvurl: String = dotenv!("SRVURL").to_owned();
    let db: String = dotenv!("DATABASE").to_owned();
    let uid: String = dotenv!("UID").to_owned();    
    let pwd: String = dotenv!("PWD").to_owned();  

    buffer = "driver={ODBC Driver 17 for SQL Server}; server=".to_owned()+ &srvurl.to_owned()+
             &"; database=".to_owned()+ &db.to_owned()+
             &"; uid=".to_owned()+ &uid.to_owned()+
             &"; pwd=".to_owned()+ &pwd.to_owned()+&";";

    io::stdin().read_line(&mut buffer).unwrap();

    let conn = env.connect_with_connection_string(&buffer)?;
    execute_statement(&conn)
}

fn execute_statement(conn: &Connection<AutocommitOn>) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;

    let mut sql_text = String::new();
    println!("Please enter SQL statement string: ");
    io::stdin().read_line(&mut sql_text).unwrap();

    match stmt.exec_direct(&sql_text)? {
        odbc::ResultSetState::Data(mut stmt) => {
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                for i in 1..(cols + 1) {
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => print!(" {}", val),
                        None => print!(" NULL"),
                    }
                }
                println!("");
            }
        }
        odbc::ResultSetState::NoData(_) => println!("Query executed, no data returned"),
    }

    Ok(())
}