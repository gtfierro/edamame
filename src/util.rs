use std::borrow::ToOwned;

#[deriving(Show)]
enum ParseError {
    InvalidCommand,
    WrongNumArguments,
}

/**
 * Parses a NICK command:
 * NICK <name>
 */
pub fn parse_nick(command : &str) -> Result<&str, ParseError> {
    let mut tokens = command.split(' ');
    if tokens.next().unwrap() != "NICK" {
        Err(ParseError::InvalidCommand)
    } else {
        match tokens.next() {
            Some(s) => Ok(s),
            None => Err(ParseError::WrongNumArguments)
        }
    }
}

/**
 * USER username 0 * :Real name
 */
pub fn parse_user(command : &str) -> Result<String, ParseError> {
    let mut tokens = command.split(' ');
    if tokens.next().unwrap() != "USER" {
        Err(ParseError::InvalidCommand)
    } else {
        tokens.next(); // skip
        tokens.next(); // skip
        tokens.next(); // skip
        let mut user = String::new();
        for tok in tokens {
            user.push_str(tok.trim_left_chars(':'));
            user.push(' ');
        };
        user.pop(); // remove trailing space

        Ok(user)
    }
}
