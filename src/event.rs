use crate::common;
use crate::common::Layout;

pub enum BspwmEvent {
	Report(ReportEvent),
	MonitorAdd(MonitorAddEvent),
	MonitorRename(MonitorRenameEvent),
	MonitorRemove(MonitorRemoveEvent),
	MonitorSwap(MonitorSwapEvent),
	MonitorFocus(MonitorFocusEvent),
	MonitorGeometry(MonitorGeometryEvent),
	DesktopAdd(DesktopAddEvent),
	DesktopRename(DesktopRenameEvent),
	DesktopRemove(DesktopRemoveEvent),
	DesktopSwap(DesktopSwapEvent),
	DesktopTransfer(DesktopTransferEvent),
	DesktopFocus(DesktopFocusEvent),
	DesktopActivate(DesktopActivateEvent),
	DesktopLayout(DesktopLayoutEvent),
	NodeAdd(NodeAddEvent),
	NodeRemove(NodeRemoveEvent),
	NodeSwap(NodeSwapEvent),
	NodeTransfer(NodeTransferEvent),
	NodeFocus(NodeFocusEvent),
	NodeActivate(NodeActivateEvent),
	NodePresel(NodePreselEvent),
	NodeStack(NodeStackEvent),
	NodeGeometry(NodeGeometryEvent),
	NodeState(NodeStateEvent),
	NodeFlag(NodeFlagEvent),
	NodeLayer(NodeLayerEvent),
	PointerAction(PointerActionEvent),
}

impl BspwmEvent {
	fn as_str(&self) -> &'static str {
		match self {
			BspwmEvent::Report(_) => "report",
			BspwmEvent::MonitorAdd(_) => "monitor_add",
			BspwmEvent::MonitorRename(_) => "monitor_rename",
			BspwmEvent::MonitorRemove(_) => "monitor_remove",
			BspwmEvent::MonitorSwap(_) => "monitor_swap",
			BspwmEvent::MonitorFocus(_) => "monitor_focus",
			BspwmEvent::MonitorGeometry(_) => "monitor_geometry",
			BspwmEvent::DesktopAdd(_) => "desktop_add",
			BspwmEvent::DesktopRename(_) => "desktop_rename",
			BspwmEvent::DesktopRemove(_) => "desktop_remove",
			BspwmEvent::DesktopSwap(_) => "desktop_swap",
			BspwmEvent::DesktopTransfer(_) => "desktop_transfer",
			BspwmEvent::DesktopFocus(_) => "desktop_focus",
			BspwmEvent::DesktopActivate(_) => "desktop_activate",
			BspwmEvent::DesktopLayout(_) => "desktop_layout",
			BspwmEvent::NodeAdd(_) => "node_add",
			BspwmEvent::NodeRemove(_) => "node_remove",
			BspwmEvent::NodeSwap(_) => "node_swap",
			BspwmEvent::NodeTransfer(_) => "node_transfer",
			BspwmEvent::NodeFocus(_) => "node_focus",
			BspwmEvent::NodeActivate(_) => "node_activate",
			BspwmEvent::NodePresel(_) => "node_presel",
			BspwmEvent::NodeStack(_) => "node_stack",
			BspwmEvent::NodeGeometry(_) => "node_geometry",
			BspwmEvent::NodeState(_) => "node_state",
			BspwmEvent::NodeFlag(_) => "node_flag",
			BspwmEvent::NodeLayer(_) => "node_layer",
			BspwmEvent::PointerAction(_) => "pointer_action",
		}
	}
}
	
pub struct ReportEvent {
	//TODO: create a type and implicit parse function.
	report: String,
}
pub struct MonitorAddEvent {
	monitor_id: u32,
	monitor_name: String,
	monitor_geometry: common::Rectangle,
}

pub struct MonitorRenameEvent {
	monitor_id: u32,
	old_name: String,
	new_name: String,
}

pub struct MonitorRemoveEvent {
	monitor_id: u32,
}

pub struct MonitorSwapEvent {
	src_monitor_id: u32,
	dst_monitor_id: u32,
}

pub struct MonitorFocusEvent {
	coordinates: u32,
}

pub struct MonitorGeometryEvent {
	coordinates: u32,
	geometry: common::Rectangle,
}

pub struct DesktopAddEvent {
	coordinates: common::DesktopCoordinates,
}

pub struct DesktopRenameEvent {
	coordinates: common::DesktopCoordinates,
	old_name: String,
	new_name: String,
}

pub struct DesktopRemoveEvent {
	coordinates: common::DesktopCoordinates,
}

pub struct DesktopSwapEvent {
	src: common::DesktopCoordinates,
	dst: common::DesktopCoordinates,
}

pub struct DesktopTransferEvent {
	coordinates: common::DesktopCoordinates,
	dst_monitor_id: u32,
}

pub struct DesktopFocusEvent {
	coordinates: common::DesktopCoordinates,
}

pub struct DesktopActivateEvent {
	coordinates: common::DesktopCoordinates,
}

pub struct DesktopLayoutEvent {
	coordinates: common::DesktopCoordinates,
	layout: Layout,
}

pub struct NodeAddEvent {
	coordinates: common::NodeCoordinates,
	node_id: u32,
}

pub struct NodeRemoveEvent {
	coordinates: common::NodeCoordinates,
}

pub struct NodeSwapEvent {
	src: common::NodeCoordinates,
	dst: common::NodeCoordinates,
}

pub struct NodeTransferEvent {
	src: common::NodeCoordinates,
	dst: common::NodeCoordinates,
}

pub struct NodeFocusEvent {
	coordinates: common::NodeCoordinates,
}

pub struct NodeActivateEvent {
	coordinates: common::NodeCoordinates,
}

pub struct NodePreselEvent {
	coordinates: common::NodeCoordinates,
	//FIXME enum with Presel type, SplitRadio type, or cancel
	presel: String,
}

pub struct NodeStackEvent {
	node_id_1: u32,
	//FIXME: not Layer just above|below
	change: String,
	node_id_2: u32,
}

pub struct NodeGeometryEvent {
	coordinates: common::NodeCoordinates,
	geometry: common::Rectangle,
}

pub struct NodeStateEvent {
	coordinates: common::NodeCoordinates,
	state: common::NodeState,
	status: bool,
}

pub struct NodeFlagEvent {
	coordinates: common::NodeCoordinates,
	flag: common::NodeFlag,
	status: bool,
}

pub struct NodeLayerEvent {
	coordinates: common::NodeCoordinates,
	layer: common::Layer,
}

pub struct PointerActionEvent {
	coordinates: common::NodeCoordinates,
	action: common::PointerAction,
	status: bool,
}
