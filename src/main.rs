extern crate rustc_serialize;


use std::io;
use rustc_serialize::json;
use std::net::{TcpStream, TcpListener, SocketAddr};
use std::io::{Read, Write, BufReader, BufRead};
use std::thread;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Message {
    user: String,
    message: String,
    users: Vec<String>,
    seqnum: u8,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8371").unwrap();    
    listener.set_nonblocking(true).expect("Can't set non-blocking");
    println!("listening started, ready to accept");
    
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            handle_request(stream);
        });
    }

}

fn handle_request(mut stream: TcpStream) {

   // let mut request_data = String::new();

   // stream.read_to_string(&mut request_data);

   // // Finally we print the data
   // println!("{}", request_data);
   // io::stdout().flush().ok().expect("Could not flush stdout");
    send_response(stream);
}

fn send_response(mut stream: TcpStream) {
    let json_to_send = Message {
        user: "Harry".to_string(),
        message: "Test Message".to_string(),
        users: vec!["Harry".to_string(), "Christian".to_string()],
        seqnum: 0,
    };

    let encoded = json::encode(&json_to_send).unwrap();
    stream.write_all(encoded.as_bytes()).unwrap();
}
