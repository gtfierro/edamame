use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

mod client;
mod server;
mod util;

fn main() {
    let mut irc = server::IRCServer::new();
    let listener = TcpListener::bind("127.0.0.1:6667").unwrap();
    let mut server = listener.listen();
    for connection in server.incoming() {
        match connection {
            Err(e) => { println!("connection failed for reason {}", e) }
            Ok(stream) => irc.handle_client(stream)
        }
    }
}
