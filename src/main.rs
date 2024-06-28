use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("localhost:0").unwrap();
    println!("Connect to {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            println!("Established connection to {}", stream.peer_addr().unwrap());
            stream
                .write(format!("Hello, {}!\n", stream.peer_addr().unwrap()).as_bytes())
                .unwrap();
            ()
        } else {
            println!("Connection failed...");
            ()
        }
    }
}
