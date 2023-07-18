use std::net::TcpStream;
use std::str;
use std::io::{Write, self, BufRead, BufReader,};


fn main() { 
    println!("Starting client module...");

    // connect to the tcp listener server 
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to TCP listener");

    loop {
        // read from client standard input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        // send data to the tcp listener server
        stream.write(input.as_bytes()).expect("Failed to write to TCP listener");

        // receive one line of data back from the tcp listener server (automatically partitions data)
        let mut reader = BufReader::new(&stream);
        let mut buf: Vec<u8> = Vec::new();
        reader.read_until(b'\n', &mut buf).expect("Failed to read from TCP listener");

        // log to client standard out
        print!("{}", str::from_utf8(&mut buf).expect("Failed to print data to client"));
    }
}