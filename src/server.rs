use std::collections::HashMap;
use std::borrow::ToOwned;
use std::io::{TcpStream};

use client::IRCClient;

pub struct IRCServer<'a> {
    users: HashMap<String, IRCClient<'a>>, // hash key = nick, value = irclient
    channels: HashMap<String, u8>
}

impl<'a> IRCServer<'a> {
    pub fn new() -> IRCServer<'a> {
        IRCServer{ users: HashMap::new(),
                   channels: HashMap::new()
                 }
    }

    pub fn handle_client(&mut self, stream: TcpStream) {
        let mut client = IRCClient::new(stream);
        client.initialize_connection();
        self.users.insert(client.nick.as_slice().to_owned(), client);
    }
}
