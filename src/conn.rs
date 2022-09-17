// DB Connection 
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use crate::utils;
// mod logging;

// #[macro_use]
// extern crate dotenv_codegen;
async fn connect_db() -> anyhow::Result<()> {
    let mut config = Config::new();

    config.host(dotenv!("SRVURL").to_string());
    config.port(dotenv!("SRVPORT").parse::<u16>().unwrap());
    config.authentication(AuthMethod::sql_server(dotenv!("SQLUSR").to_string(), dotenv!("SQLPASS").to_string()));
    config.instance_name(dotenv!("INSTANCE").to_string());
    config.trust_cert(); // on production, it is not a good idea to do this

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // To be able to use Tokio's tcp, we're using the `compat_write` from
    // the `TokioAsyncWriteCompatExt` to get a stream compatible with the
    // traits from the `futures` crate.
    let _client = Client::connect(config, tcp.compat_write()).await?;

    Ok(())
}

pub async fn connect() {
    // println!("Connecting to DB {}", dotenv!("SRVURL").to_string());
    connect_db().await;

    // ERROR Throw to file
    if let Err(_err) = connect_db().await {
        utils::oplog("\n ERROR: ".to_owned() + &_err.to_string()).await;
        println!("ERROR {}", _err.to_string());
    }
} 