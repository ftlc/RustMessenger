extern crate rustc_serialize;
extern crate mio;


use std::io;
use rustc_serialize::json;
use std::net::{TcpStream, SocketAddr};
use std::net::TcpListener;
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
    println!("listening started, ready to accept");
    
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream.set_read_timeout(None).expect("set_read_timeout call failed");
        stream.set_nodelay(true).expect("set_nodelay call failed");
        stream.set_write_timeout(None).expect("set_write_timeout call failed");
        thread::spawn(|| {
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
