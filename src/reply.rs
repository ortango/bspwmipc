#![allow(warnings)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct bspwmstate_t {
	pub clientsCount: u32,
	pub focusHistory: Vec<coordinates_t>,
	pub focusedMonitorId: u32,
	pub monitors: Vec<monitor_t>,
	pub primaryMonitorId: u32,
	pub stackingList: Vec<u32>,
}

#[derive(Deserialize, Debug)]
pub struct monitor_t {
	pub borderWidth: u16,
	pub desktops: Vec<desktop_t>,
	pub focusedDesktopId: u32,
	pub id: u32,
	pub name: String,
	pub padding: padding_t,
	pub randrId: u32,
	pub rectangle: rectangle_t,
	pub stickyCount: u32,
	pub windowGap: i32,
	pub wired: bool
}

#[derive(Deserialize, Debug)]
pub struct desktop_t {
	pub borderWidth: u16,
	pub focusedNodeId: u32,
	pub id: u32,
	pub layout: layout_t,
	pub name: String,
	pub padding: padding_t,
	pub root: Option<node_t>,
	pub userLayout: layout_t,
	pub windowGap: i32
}

#[derive(Deserialize, Debug)]
pub struct node_t {
	pub client: Option<client_t>,
	pub constraints: constraints_t,
	pub firstChild: Option<Box<node_t>>,
	pub hidden: bool,
	pub id: u32,
	pub locked: bool,
	pub marked: bool,
	pub presel: Option<presel_t>,
	pub private: bool,
	pub rectangle: rectangle_t,
	pub secondChild: Option<Box<node_t>>,
	pub splitRatio: f64,
	pub splitType: splittype_t,
	pub sticky: bool,
	pub vacant: bool
}

impl node_t {
	pub fn traverse(&self) -> Vec<&node_t> {
		let mut stack: Vec<&node_t> = Vec::new();
		let mut res: Vec<&node_t> = Vec::new();
		stack.push(self);
		while !stack.is_empty() {
			let node = stack.pop().unwrap();
			res.push(node);
			match node.firstChild {
				None => {}
				Some(ref n) => stack.push(n),
			}
			match node.secondChild {
				None => {}
				Some(ref n) => stack.push(n),
			}
		}
		res
	}
}

#[derive(Deserialize, Debug)]
pub struct client_t {
	pub borderWidth: u16,
	pub className: String,
	pub floatingRectangle: rectangle_t,
	pub instanceName: String,
	pub lastLayer: layer_t,
	pub lastState: state_t,
	pub layer: layer_t,
	pub shown: bool,
	pub state: state_t,
	pub tiledRectangle: rectangle_t,
	pub urgent: bool
}

impl client_t {
	pub fn getgeometry(&self) -> &rectangle_t {
		if self.state == state_t::floating {
			return &self.floatingRectangle;
		}
		return &self.tiledRectangle;
	}
}

#[derive(Deserialize, Debug)]
pub struct presel_t {
	pub splitDir: direction_t,
	pub splitRatio: f64
}

#[derive(Deserialize, Debug)]
pub struct padding_t {
	pub top: i16,
	pub right: i16,
	pub bottom: i16,
	pub left: i16
}

#[derive(Deserialize, Debug)]
pub struct rectangle_t {
	pub x: i16,
	pub y: i16,
	pub width: u16,
	pub height: u16
}

#[derive(Deserialize, Debug)]
pub struct constraints_t {
	pub min_width: u16,
	pub min_height: u16
}

#[derive(Deserialize, Debug)]
pub struct coordinates_t {
	pub monitorId: u32,
	pub desktopId: u32,
	pub nodeId: u32
}

#[derive(Deserialize, Debug)]
pub enum layout_t {
	tiled,
	monocle
}

#[derive(Deserialize, Debug)]
pub enum splittype_t {
	horizontal,
	vertical
}

#[derive(Deserialize, Debug)]
pub enum direction_t {
	north,
	west,
	south,
	east
}

#[derive(Deserialize, Debug)]
pub enum layer_t {
	below,
	normal,
	above
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum state_t {
	tiled,
	pseudo_tiled,
	floating,
	fullscreen
}
