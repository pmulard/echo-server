use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};


fn handle_client(mut stream: TcpStream) {
    println!("Handling client from {}", stream.peer_addr().unwrap());
    // set buffer size to partition data while processing
    let mut buf = [0; 1024];

    // read and write data until there is none left from client
    loop {
        let data = stream.read(&mut buf).expect("Failed to read from client");
        if data == 0 { return };
        stream.write(&buf[..data]).expect("Failed to write to client");
    }
}

fn main() {
    // start a tcp listener on an open socket
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to start TCP listener server");

    // accept connections sequentially and process them in parallel
    for stream in listener.incoming() {
        match stream {
            Err(e) => {eprintln!("Stream failed...\n{}", e)}
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
        }
    }
}
