use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::error::Error;
use std::fmt;
use std::sync::Mutex;

use crate::store::Store;

#[derive(Debug)]
enum CommandError {
    InvalidError(String),
    NotRecognizedError(String)
}

impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            CommandError::InvalidError(err) => {
                write!(f, "{}", err)
            },
             CommandError::NotRecognizedError(err) => {
                write!(f, "{}", err)
            }
        }

    }
}

#[derive(Debug)]
enum Command {
    Get(String),
    Set(String, String),
    Del(String)
}

impl Command {
    fn parse(input: String) -> Result<Self, CommandError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(CommandError::InvalidError("command invalid".to_string()))
        }

        match parts[0] {
            "GET" => {
                Ok(Command::Get(parts[1].to_string()))
            },
            "SET" => {
                if parts.len() < 3 {
                    Err(CommandError::InvalidError("command invalid".to_string()))
                } else {
                    Ok(Command::Set(parts[1].to_string(), parts[2..].join(" ").to_string()))
                }
            },
            "DEL" => {
                Ok(Command::Del(parts[1].to_string()))
            },
            _ => Err(CommandError::NotRecognizedError("command not recognized".to_string()))
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Get(key) => write!(f, "GET: {}", key),
            Command::Set(key, value) => write!(f, "SET: {} {}", key, value),
            Command::Del(key) => write!(f, "DEL: {}", key),
        }
    }
}

pub struct Server {
    port: u16,
    store: Mutex<Store>
}

impl Server {
    pub fn new(port: u16, store: Store) -> Self {
        Server {
            port,
            store: store.into()
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let address = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(address)?;

        for stream in listener.incoming() {
            self.handle_client(stream?);
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buf = [0u8; 512];
        loop {
            match stream.read(&mut buf) {
                Ok(bytes_read) => {
                    // handle the data read from the client here
                    let input = String::from_utf8_lossy(&buf[..bytes_read]);
                    let response = match Command::parse(input.to_string()) {
                        Ok(command) => {
                            match command {
                                Command::Get(key) => {
                                    match self.store.lock().unwrap().get(&key) {
                                        Some(val) => {
                                            format!("key {}: got val {}\n", key, val)
                                        },
                                        None => {
                                            format!("key {}: not found\n", key)
                                        }
                                    }
                                },
                                Command::Set(key, val) => {
                                    match self.store.lock().unwrap().set(&key, &val) {
                                        Some(result_val) => {
                                            format!("key {}: set to val: {}, old val: {}\n", key, val, result_val)
                                        },
                                        None => {
                                            format!("key {}: set to val: {}\n", key, val)
                                        }
                                    }
                                },
                                Command::Del(key) => {
                                    match self.store.lock().unwrap().del(&key) {
                                        Some(result_val) => {
                                            format!("key {}: was deleted, val is {}\n", key, result_val)
                                        },
                                        None => {
                                            format!("key {}: not found\n", key)
                                        }
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            format!("Failed to parse command: {}\n", e)
                        }
                    };
                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        eprintln!("Failed to write command to client: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from client: {}", e);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parse_get_success() {
        let input = "GET something".to_string();
        if let Ok(parsed) = Command::parse(input.clone()) {
            assert!(matches!(parsed, Command::Get(ref k) if k == "something"));
        } else {
            panic!("expected parse to succeed on `{}`", input);
        }
    }

    #[test]
    fn test_command_parse_del_success() {
        let input = "DEL something".to_string();
        if let Ok(parsed) = Command::parse(input.clone()) {
            assert!(matches!(parsed, Command::Del(ref k) if k == "something"));
        } else {
            panic!("expected parse to succeed on `{}`", input);
        }
    }

    #[test]
    fn test_command_parse_set_success() {
        let input = "SET something something else is wrong with the world".to_string();
        if let Ok(parsed) = Command::parse(input.clone()) {
            assert!(matches!(parsed, Command::Set(ref k, ref v) if k == "something" && v == "something else is wrong with the world"));
        } else {
            panic!("expected parse to succeed on `{}`", input);
        }
    }

    #[test]
    fn test_command_parse_with_extra_whitespace() {
        let input = "SET    something    something world  nice".to_string();
        if let Ok(parsed) = Command::parse(input.clone()) {
            assert!(matches!(parsed, Command::Set(ref k, ref v) if k == "something" && v == "something world nice"));
        } else {
            panic!("expected parse to succeed on `{}`", input);
        }
    }

    #[test]
    fn test_command_parse_empty_string_is_invalid() {
        let input = "".to_string();
        if let Err(error) = Command::parse(input.clone()) {
            assert!(matches!(error, CommandError::InvalidError(ref k) if k == "command invalid"))
        } else {
            panic!("expected parse to fail on `{}`", input);
        }
    }


    #[test]
    fn test_command_parse_unrecognized_command() {
        let input = "HEEHEE hee".to_string();
        if let Err(error) = Command::parse(input.clone()) {
            assert!(matches!(error, CommandError::NotRecognizedError(ref k) if k == "command not recognized"))
        } else {
            panic!("expected parse to fail on `{}`", input);
        }
    }

    #[test]
    fn test_command_parse_invalid_set_command() {
        let input = "SET invalid".to_string();
        if let Err(error) = Command::parse(input.clone()) {
            assert!(matches!(error, CommandError::InvalidError(ref k) if k == "command invalid"))
        } else {
            panic!("expected parse to fail on `{}`", input);
        }
    }

    #[test]
    fn test_command_parse_ensure_case_sensitive() {
        let input = "get hello".to_string();
        if let Err(error) = Command::parse(input.clone()) {
            assert!(matches!(error, CommandError::NotRecognizedError(ref k) if k == "command not recognized"))
        } else {
            panic!("expected parse to fail on `{}`", input);
        }
    }
}
