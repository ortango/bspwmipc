use std::error::Error;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::{env, fmt, io};
extern crate serde;
pub mod reply;
pub mod event;
pub mod common;
use reply::{BspwmTree, BspwmState, Desktop, Monitor, Node};

const BUFSIZ: usize = 8192;
const SOCKET_ENV_VAR: &str = "BSPWM_SOCKET";
const FAILURE_MESSAGE: i32 = 7;

#[derive(Debug)]
pub enum MessageError {
    Send(io::Error),
    Receive(io::Error),
    JsonCouldntParse(serde_json::Error),
    Status(io::Error),
}

impl Error for MessageError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            MessageError::Send(ref e) => Some(e),
            MessageError::Receive(ref e) => Some(e),
            MessageError::JsonCouldntParse(ref e) => Some(e),
            MessageError::Status(ref e) => Some(e),
        }
    }
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageError::Send(_) => write!(f, "Network error while sending message to bspwm"),
            MessageError::Receive(_) => {
                write!(f, "Network error while receiving message from bspwm")
            }
            MessageError::JsonCouldntParse(_) => {
                write!(f, "Got a response from bspwm but couldn't parse the JSON")
            }
            MessageError::Status(_) => write!(f, "Bspwm command failed"),
        }
    }
}

fn get_socket_path() -> String {
    match env::var(SOCKET_ENV_VAR) {
	    Ok(p) => p,
	    Err(_) => format!("/tmp/bspwm{}_{}_{}-socket", "", 0, 0),
    }
}

trait BspwmFuncs {
    fn send_bspwm_message(&mut self, payload: &str) -> io::Result<()>;
    fn receive_bspwm_message(&mut self) -> io::Result<(String, bool)>;
}

impl BspwmFuncs for UnixStream {
    fn send_bspwm_message(&mut self, message: &str) -> io::Result<()> {
        let mut i = 0;
        let mut buffer = [0; BUFSIZ];
        for s in message.split_whitespace() {
            for c in s.chars() {
                buffer[i] = c as u8;
                i += 1;
            }
            buffer[i] = 0;
            i += 1;
        }
        self.write_all(&buffer[..i])
    }
    // bufreader still probably needed because of subscribe. unless it would be seperated out.
    // boo.
    fn receive_bspwm_message(&mut self) -> io::Result<(String, bool)> {
        let mut buffer = Vec::new();
        let mut status: bool = true;
        let mut response: String = "".to_string();
        let size = self.read_to_end(&mut buffer)?;
        if size > 0 {
            let offset = match buffer[0] as i32 {
                FAILURE_MESSAGE => 1,
                _ => 0,
            };
            response = String::from_utf8_lossy(&buffer[offset..size]).into_owned();
            status = offset == 0;
        }
        Ok((response, status))
    }
}

#[derive(Debug)]
pub struct BspwmConnection {
    socket_path: String,
}

impl Default for BspwmConnection {
	fn default() -> BspwmConnection {
		BspwmConnection {
			socket_path: get_socket_path(),
		}
	}
}

impl BspwmConnection {
	pub fn new() -> Self{
		BspwmConnection::default()
	}
    fn send_receive_bspwm_message(&mut self, message: &str) -> Result<String, MessageError> {
	    let mut stream = match UnixStream::connect(&self.socket_path) {
		    Ok(s) => s,
		    Err(e) => return Err(MessageError::Send(e)),
	    };
        if let Err(e) = stream.send_bspwm_message(message) {
            return Err(MessageError::Send(e));
        }
        let response = match stream.receive_bspwm_message() {
            Ok((r, s)) => match s {
                true => r,
                false => {
                    return Err(
	                    MessageError::Status(io::Error::new(io::ErrorKind::Other, r,))
                    )
                }
            },
            Err(e) => return Err(MessageError::Receive(e)),
        };
        Ok(response)
    }
    pub fn raw_command(&mut self, message: &str) -> Result<String, MessageError> {
        self.send_receive_bspwm_message(message)
    }
    fn get_json<T: serde::de::DeserializeOwned>(
        &mut self,
        message: &str,
    ) -> Result<T, MessageError> {
        let response = self.send_receive_bspwm_message(message)?;
        match serde_json::from_str(&response) {
            Ok(v) => Ok(v),
            Err(e) => Err(MessageError::JsonCouldntParse(e)),
        }
    }
    pub fn get_bspwm_state(&mut self) -> Result<BspwmState, MessageError> {
        self.get_json(BspwmTree::Bspwm.as_str())
    }
    pub fn get_monitor(&mut self, id: &u32) -> Result<Monitor, MessageError> {
        self.get_json(&(format!("{} {}", BspwmTree::Monitor.as_str(), &id)))
    }
    pub fn get_desktop(&mut self, id: &u32) -> Result<Desktop, MessageError> {
        self.get_json(&(format!("{} {}", BspwmTree::Desktop.as_str(), &id)))
    }
    pub fn get_node(&mut self, id: &u32) -> Result<Node, MessageError> {
        self.get_json(&(format!("{} {}", BspwmTree::Node.as_str(), &id)))
    }
}
