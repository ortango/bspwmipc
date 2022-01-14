use std::error::Error;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::{env, fmt, io};
extern crate serde;
pub mod reply;
use reply::{BspwmState, Desktop, Monitor, Node};

const BUFSIZ: usize = 8192;
const SOCKET_ENV_VAR: &str = "BSPWM_SOCKET";
const FAILURE_MESSAGE: i32 = 7;

#[derive(Debug)]
pub enum EstablishError {
    GetSocketPathError(io::Error),
    SocketError(io::Error),
}

impl Error for EstablishError {
    fn description(&self) -> &str {
        match *self {
            EstablishError::GetSocketPathError(_) => "Couldn't determine bspwm's socket path",
            EstablishError::SocketError(_) => "Found bspwm's socket path but failed to connect",
        }
    }
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            EstablishError::GetSocketPathError(ref e) | EstablishError::SocketError(ref e) => {
                Some(e)
            }
        }
    }
}

impl fmt::Display for EstablishError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub enum MessageError {
    Send(io::Error),
    Receive(io::Error),
    JsonCouldntParse(serde_json::Error),
}

impl Error for MessageError {
    fn description(&self) -> &str {
        match *self {
            MessageError::Send(_) => "Network error while sending message to bspwm",
            MessageError::Receive(_) => "Network error while receiving message from bspwm",
            MessageError::JsonCouldntParse(_) => {
                "Got a response from bspwm but couldn't parse the JSON"
            }
        }
    }
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            MessageError::Send(ref e) => Some(e),
            MessageError::Receive(ref e) => Some(e),
            MessageError::JsonCouldntParse(ref e) => Some(e),
        }
    }
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

//TODO: correctly set fallback socket value
fn get_socket_path() -> io::Result<String> {
    if let Ok(sockpath) = env::var(SOCKET_ENV_VAR) {
        return Ok(sockpath);
    }
    return Ok(format!("/tmp/bspwm{}_{}_{}-socket", "", 0, 0));
}

trait BspwmFuncs {
    fn send_bspwm_message(&mut self, payload: &str) -> io::Result<()>;
    fn receive_bspwm_message(&mut self) -> io::Result<String>;
    fn send_receive_bspwm_message(&mut self, message: &str) -> Result<String, MessageError>;
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
    fn receive_bspwm_message(&mut self) -> io::Result<String> {
        let mut buffer = Vec::new();
        let size = self.read_to_end(&mut buffer)?;
        buffer.push(0);
        let offset = match buffer[0] as i32 {
            FAILURE_MESSAGE => 1,
            _ => 0,
        };
        let response = String::from_utf8_lossy(&buffer[offset..size]).into_owned();
        match offset {
            1 => Err(io::Error::new(io::ErrorKind::Other, response)),
            _ => Ok(response),
        }
    }
    fn send_receive_bspwm_message(&mut self, message: &str) -> Result<String, MessageError> {
        if let Err(e) = self.send_bspwm_message(message) {
            return Err(MessageError::Send(e));
        }
        let response = match self.receive_bspwm_message() {
            Ok(message) => message,
            Err(e) => {
                return Err(MessageError::Receive(e));
            }
        };
        Ok(response)
    }
}

#[derive(Debug)]
pub struct BspwmConnection {
    stream: UnixStream,
}

impl BspwmConnection {
    pub fn connect() -> Result<BspwmConnection, EstablishError> {
        match get_socket_path() {
            Ok(path) => match UnixStream::connect(path) {
                Ok(stream) => Ok(BspwmConnection { stream }),
                Err(error) => Err(EstablishError::SocketError(error)),
            },
            Err(error) => Err(EstablishError::GetSocketPathError(error)),
        }
    }
    pub fn raw_command(&mut self, message: &str) -> Result<String, MessageError> {
        self.stream.send_receive_bspwm_message(message)
    }
    fn get_json<T: serde::de::DeserializeOwned>(
        &mut self,
        message: &str,
    ) -> Result<T, MessageError> {
        let response = self.stream.send_receive_bspwm_message(message)?;
        match serde_json::from_str(&response) {
            Ok(v) => Ok(v),
            Err(e) => Err(MessageError::JsonCouldntParse(e)),
        }
    }
    pub fn get_bspwm_state(&mut self) -> Result<BspwmState, MessageError> {
        self.get_json("wm -d")
    }
    pub fn get_monitor(&mut self, id: &u32) -> Result<Monitor, MessageError> {
        self.get_json(&(format!("query -T -m {}", &id)))
    }
    pub fn get_desktop(&mut self, id: &u32) -> Result<Desktop, MessageError> {
        self.get_json(&(format!("query -T -d {}", &id)))
    }
    pub fn get_node(&mut self, id: &u32) -> Result<Node, MessageError> {
        self.get_json(&(format!("query -T -n {}", &id)))
    }
}
