use serde::Deserialize;

pub struct DesktopCoordinates {
	monitor_id: u32,
	id: u32,
}

pub enum NodeFlag {
	Hidden,
	Sticky,
	Private,
	Locked,
	Marked,
	Urgent,
}

pub enum PointerAction {
	Move,
	ResizeCorner,
	ResizeSide,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Rectangle {
	pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct NodeCoordinates {
	pub monitor_id: u32,
    pub desktop_id: u32,
    pub node_id: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Layout {
    Tiled,
    Monocle,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Layer {
    Below,
    Normal,
    Above,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum NodeState {
    Tiled,
    PseudoTiled,
    Floating,
    Fullscreen,
}
