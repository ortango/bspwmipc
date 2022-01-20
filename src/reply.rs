use serde::Deserialize;
use crate::common;
use crate::common::{Layer,Rectangle};

pub enum BspwmTree {
	Bspwm,
	Monitor,
	Desktop,
	Node,
}

impl BspwmTree {
	pub fn as_str(&self) -> &'static str {
		match self {
			BspwmTree::Bspwm => "wm -d",
			BspwmTree::Monitor => "query -T -m",
			BspwmTree::Desktop => "query -T -d",
			BspwmTree::Node => "query -T -n",
		}
	}
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BspwmState {
    pub clients_count: u32,
    pub focus_history: Vec<common::NodeCoordinates>,
    pub focused_monitor_id: u32,
    pub monitors: Vec<Monitor>,
    pub primary_monitor_id: u32,
    pub stacking_list: Vec<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Monitor {
    pub border_width: u16,
    pub desktops: Vec<Desktop>,
    pub focused_desktop_id: u32,
    pub id: u32,
    pub name: String,
    pub padding: Padding,
    pub randr_id: u32,
    pub rectangle: Rectangle,
    pub sticky_count: u32,
    pub window_gap: i32,
    pub wired: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Desktop {
    pub border_width: u16,
    pub focused_node_id: u32,
    pub id: u32,
    pub layout: common::Layout,
    pub name: String,
    pub padding: Padding,
    pub root: Option<Node>,
    pub user_layout: common::Layout,
    pub window_gap: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Node {
    pub client: Option<Client>,
    pub constraints: Constraints,
    pub first_child: Option<Box<Node>>,
    pub hidden: bool,
    pub id: u32,
    pub locked: bool,
    pub marked: bool,
    pub presel: Option<Presel>,
    pub private: bool,
    pub rectangle: Rectangle,
    pub second_child: Option<Box<Node>>,
    pub split_ratio: f64,
    pub split_type: SplitType,
    pub sticky: bool,
    pub vacant: bool,
}

impl Node {
    pub fn traverse(&self) -> Vec<&Node> {
        let mut stack: Vec<&Node> = Vec::new();
        let mut res: Vec<&Node> = Vec::new();
        stack.push(self);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            res.push(node);
            match node.first_child {
                None => {}
                Some(ref n) => stack.push(n),
            }
            match node.second_child {
                None => {}
                Some(ref n) => stack.push(n),
            }
        }
        res
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Client {
    pub border_width: u16,
    pub class_name: String,
    pub floating_rectangle: common::Rectangle,
    pub instance_name: String,
    pub last_layer: common::Layer,
    pub last_state: common::NodeState,
    pub layer: Layer,
    pub shown: bool,
    pub state: common::NodeState,
    pub tiled_rectangle: common::Rectangle,
    pub urgent: bool,
}

impl Client {
    pub fn get_geometry(&self) -> &Rectangle {
        match self.state {
            common::NodeState::Floating => &self.floating_rectangle,
            _ => &self.tiled_rectangle,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Presel {
    pub split_dir: Direction,
    pub split_ratio: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Padding {
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
    pub left: i16,
}

#[derive(Deserialize, Debug)]
pub struct Constraints {
    pub min_width: u16,
    pub min_height: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum SplitType {
    Horizontal,
    Vertical,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Direction {
    North,
    West,
    South,
    East,
}
