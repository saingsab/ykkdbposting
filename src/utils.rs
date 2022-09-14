// Libs and Utilitis to serve in the operation
use std::fs::OpenOptions;
use std::io::Write;

struct Msg {
    error_msg: String,
}

impl Msg {
    fn new (error_msg: &string) -> Msg {
        Msg {err_msg: err_msg.to_string()}
    }
}

// [ ] syslog: loging any error occure when fail running the system.
// [ ] oplog: loging any error occure when fail running the operation.
pub fn oplog() {
    let mut file_ref = OpenOptions::new().append(true).open("log/op.log").expect("Unable to open file");   
    
    file_ref.write_all("ERROR : {}\n", err_msg.as_bytes()).expect("write failed");
    println!("Log appended successfully");
 }