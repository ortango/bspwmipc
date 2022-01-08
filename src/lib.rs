use std::error::Error;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::{env, fmt, io};
extern crate serde;
pub mod reply;
use reply::{bspwmstate_t,monitor_t,desktop_t,node_t};

const BUFSIZ: usize = 8192 as usize;
const SOCKET_ENV_VAR: &'static str = "BSPWM_SOCKET";

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
		write!(f, "{}", self.to_string())
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
	fn send_receive_bspwm_message(&mut self, message: &str) -> String;
}

impl BspwmFuncs for UnixStream {
	fn send_receive_bspwm_message(&mut self, message: &str) -> String {
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
		self.write(&buffer[..i]).unwrap();
		let mut response = String::new();
		self.read_to_string(&mut response).expect("failed to communicate with bspwm.");
		return response
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
	// TODO: add support of status (all) and possible string return.
	pub fn raw_command(&mut self, message: &str) {
		let _reply = self.stream.send_receive_bspwm_message(&message);
	}
	pub fn get_bspwm_state(&mut self) -> bspwmstate_t {
		let reply = self.stream.send_receive_bspwm_message("wm -d");
		let state = serde_json::from_str(&reply).unwrap();
		return state
	}
	pub fn get_monitor(&mut self, id: &u32) -> monitor_t {
		let reply = self.stream.send_receive_bspwm_message(&("query -T -m ".to_owned() + &id.to_string()));
		let monitor = serde_json::from_str(&reply).unwrap();
		return monitor
	}
	pub fn get_desktop(&mut self, id: &u32) -> desktop_t {
		let reply = self.stream.send_receive_bspwm_message(&("query -T -d ".to_owned() + &id.to_string()));
		let desktop = serde_json::from_str(&reply).unwrap();
		return desktop
	}
	pub fn get_node(&mut self, id: &u32) -> node_t {
		let reply = self.stream.send_receive_bspwm_message(&("query -T -n ".to_owned() + &id.to_string()));
		let node = serde_json::from_str(&reply).unwrap();
		return node
	}
}
