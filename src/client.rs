use std::io::{TcpStream};
use std::borrow::ToOwned;
use std::str;
use util;

pub struct IRCClient<'a> {
    conn: TcpStream,
    pub nick: String,
    fullname: String
}

impl<'a> IRCClient<'a> {
    /** take a new TCP connection and create IRC client **/
    pub fn new(mut connection: TcpStream) -> IRCClient<'a> {
        match connection.peer_name() {
            Ok(addr) => {
                println!("Got connection from {}:{}", addr.ip, addr.port);
                IRCClient{ conn: connection, nick: String::new(), fullname: String::new()}
                },
            Err(e) => {
                println!("Couldn't get peer name? {}", e);
                IRCClient{ conn: connection, nick: String::new(), fullname: String::new()}
                }
        }
    }

    /**
     * This handles the NICK and USER
     */
    pub fn initialize_connection(&mut self) {
        let mut readbuffer :[u8, ..512] = [0, ..512];
        let numbytes = match self.conn.read(&mut readbuffer) {
            Ok(numbytes) => numbytes,
            Err(_) => 0,
        };
        if numbytes > 0 {
            let s = match str::from_utf8(readbuffer.as_slice()) {
                Ok(s) => s,
                Err(e) => { println!("Error: {}", e);
                            "" }
            };

            // divide up the commands
            let mut commands = s.split('\n');

            for cmd in commands {
                println!("command: {}", cmd);
                // get the NICK name
                if cmd.starts_with("NICK") {
                    let nick = match util::parse_nick(cmd) {
                        Ok(s) => s,
                        Err(_) => "a"
                    };
                    self.nick = nick.to_owned();
                } else if cmd.starts_with("USER") {
                // get full name
                    let fullname = match util::parse_user(cmd) {
                        Ok(s) => s,
                        Err(_) => String::from_str("Default")
                    };
                    self.fullname = fullname;
                } else {
                }
            }
        }

        println!("Got nick {}", self.nick);
        println!("Got fullname {}", self.fullname);
    }
}

