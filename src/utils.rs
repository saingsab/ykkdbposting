// Libs and Utilitis to serve in the operation
use std::fs::OpenOptions;
use std::io::Write;

// [x] syslog: loging any error occure when fail running the system.
pub fn syslog(_msg: String) {
    let mut file_ref = OpenOptions::new().append(true).open("log/sys.log").expect("Unable to open file");   
    
    file_ref.write_all(_msg.as_bytes()).expect("write failed");
    println!("Log appended successfully");
 }

// [x] oplog: loging any error occure when fail running the operation.
pub fn oplog(_msg: String) {
    let mut file_ref = OpenOptions::new().append(true).open("log/op.log").expect("Unable to open file");   
    
    file_ref.write_all(_msg.as_bytes()).expect("write failed");
    println!("Log appended successfully");
 }