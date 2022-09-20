// DB Connection 
// use tiberius::{Client, Config, AuthMethod};
// use tokio::net::TcpStream;
// use tokio_util::compat::TokioAsyncWriteCompatExt;

// use crate::utils;
// // mod logging;

// // #[macro_use]
// // extern crate dotenv_codegen;
// async fn connect_db() -> anyhow::Result<()> {
//     let mut config = Config::new();

//     config.host(dotenv!("SRVURL").to_string());
//     config.port(dotenv!("SRVPORT").parse::<u16>().unwrap());
//     config.authentication(AuthMethod::sql_server(dotenv!("SQLUSR").to_string(), dotenv!("SQLPASS").to_string()));
//     config.instance_name(dotenv!("INSTANCE").to_string());
//     config.trust_cert(); // on production, it is not a good idea to do this

//     let tcp = TcpStream::connect(config.get_addr()).await?;
//     tcp.set_nodelay(true)?;

//     // To be able to use Tokio's tcp, we're using the `compat_write` from
//     // the `TokioAsyncWriteCompatExt` to get a stream compatible with the
//     // traits from the `futures` crate.
//     let _client = Client::connect(config, tcp.compat_write()).await?;

//     Ok(())
// }

// pub async fn connect() {
//     // println!("Connecting to DB {}", dotenv!("SRVURL").to_string());
//     connect_db().await;

//     // ERROR Throw to file
//     if let Err(_err) = connect_db().await {
//         utils::oplog("\n ERROR: ".to_owned() + &_err.to_string()).await;
//         println!("ERROR {}", _err.to_string());
//     }
// } 

// ODBC Driver Connection Here
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