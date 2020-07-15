use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};

fn main() {
    
    let input = TcpListener::bind(SocketAddr::from(([127,0,0,1], 5432))).unwrap();
    let mut output = TcpStream::connect(SocketAddr::from(([172,20,129,75], 5432))).unwrap();
    for stream in input.incoming() {
        if let Ok(mut stream) = stream {
            let mut buf = Vec::<u8>::new();
            let _ = stream.read_to_end(&mut buf);
            println!("{:?}", buf);
            let _ = output.write_all(buf.as_slice());
        }
    }

}
