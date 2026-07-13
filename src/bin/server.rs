use std::io;
use std::net::{TcpListener};
use std::io::{Read};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6767").expect("Failed to bind to address");
    println!("Server listening on PORT 6767");

    
}

