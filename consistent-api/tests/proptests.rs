use std::assert_matches;
use std::collections::HashMap;
use proptest::prelude::*;
use proptest_macro::property_test;
use atspi::Role;

#[allow(clippy::too_many_lines)]
fn role() -> impl Strategy<Value = Role> {
	prop_oneof![
		Just(Role::Invalid),
		Just(Role::AcceleratorLabel),
		Just(Role::Alert),
		Just(Role::Animation),
		Just(Role::Arrow),
		Just(Role::Calendar),
		Just(Role::Canvas),
		Just(Role::CheckBox),
		Just(Role::CheckMenuItem),
		Just(Role::ColorChooser),
		Just(Role::ColumnHeader),
		Just(Role::ComboBox),
		Just(Role::DateEditor),
		Just(Role::DesktopIcon),
		Just(Role::DesktopFrame),
		Just(Role::Dial),
		Just(Role::Dialog),
		Just(Role::DirectoryPane),
		Just(Role::DrawingArea),
		Just(Role::FileChooser),
		Just(Role::Filler),
		Just(Role::FocusTraversable),
		Just(Role::FontChooser),
		Just(Role::Frame),
		Just(Role::GlassPane),
		Just(Role::HTMLContainer),
		Just(Role::Icon),
		Just(Role::Image),
		Just(Role::InternalFrame),
		Just(Role::Label),
		Just(Role::LayeredPane),
		Just(Role::List),
		Just(Role::ListItem),
		Just(Role::Menu),
		Just(Role::MenuBar),
		Just(Role::MenuItem),
		Just(Role::OptionPane),
		Just(Role::PageTab),
		Just(Role::PageTabList),
		Just(Role::Panel),
		Just(Role::PasswordText),
		Just(Role::PopupMenu),
		Just(Role::ProgressBar),
		Just(Role::Button),
		Just(Role::RadioButton),
		Just(Role::RadioMenuItem),
		Just(Role::RootPane),
		Just(Role::RowHeader),
		Just(Role::ScrollBar),
		Just(Role::ScrollPane),
		Just(Role::Separator),
		Just(Role::Slider),
		Just(Role::SpinButton),
		Just(Role::SplitPane),
		Just(Role::StatusBar),
		Just(Role::Table),
		Just(Role::TableCell),
		Just(Role::TableColumnHeader),
		Just(Role::TableRowHeader),
		Just(Role::TearoffMenuItem),
		Just(Role::Terminal),
		Just(Role::Text),
		Just(Role::ToggleButton),
		Just(Role::ToolBar),
		Just(Role::ToolTip),
		Just(Role::Tree),
		Just(Role::TreeTable),
		Just(Role::Unknown),
		Just(Role::Viewport),
		Just(Role::Window),
		Just(Role::Extended),
		Just(Role::Header),
		Just(Role::Footer),
		Just(Role::Paragraph),
		Just(Role::Ruler),
		Just(Role::Application),
		Just(Role::Autocomplete),
		Just(Role::Editbar),
		Just(Role::Embedded),
		Just(Role::Entry),
		Just(Role::CHART),
		Just(Role::Caption),
		Just(Role::DocumentFrame),
		Just(Role::Heading),
		Just(Role::Page),
		Just(Role::Section),
		Just(Role::RedundantObject),
		Just(Role::Form),
		Just(Role::Link),
		Just(Role::InputMethodWindow),
		Just(Role::TableRow),
		Just(Role::TreeItem),
		Just(Role::DocumentSpreadsheet),
		Just(Role::DocumentPresentation),
		Just(Role::DocumentText),
		Just(Role::DocumentWeb),
		Just(Role::DocumentEmail),
		Just(Role::Comment),
		Just(Role::ListBox),
		Just(Role::Grouping),
		Just(Role::ImageMap),
		Just(Role::Notification),
		Just(Role::InfoBar),
		Just(Role::LevelBar),
		Just(Role::TitleBar),
		Just(Role::BlockQuote),
		Just(Role::Audio),
		Just(Role::Video),
		Just(Role::Definition),
		Just(Role::Article),
		Just(Role::Landmark),
		Just(Role::Log),
		Just(Role::Marquee),
		Just(Role::Math),
		Just(Role::Rating),
		Just(Role::Timer),
		Just(Role::Static),
		Just(Role::MathFraction),
		Just(Role::MathRoot),
		Just(Role::Subscript),
		Just(Role::Superscript),
		Just(Role::DescriptionList),
		Just(Role::DescriptionTerm),
		Just(Role::DescriptionValue),
		Just(Role::Footnote),
		Just(Role::ContentDeletion),
		Just(Role::ContentInsertion),
		Just(Role::Mark),
		Just(Role::Suggestion),
		Just(Role::PushButtonMenu),
	]
}

fn node() -> impl Strategy<Value = Node> {
	(any::<u32>().prop_map(NodeId), role(), any::<u32>(), any::<String>(), proptest::collection::vec(any::<u32>().prop_map(NodeId), 0..100)).prop_map(|(id, role, parent, text, children)| {
		Node {
			id,
			text,
			children,
			role,
			parent: NodeId(parent),
		}
	})
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct NodeId(u32);

#[derive(Default)]
pub struct NodeMap {
	tree: HashMap<NodeId, Node>,
	disconnected: HashMap<NodeId, Node>,
}
impl NodeMap {
	fn apply(&mut self, change: ChangeItems) {
		match change {
			ChangeItems::AddNode(nodes) => {
				for node in nodes {
					self.tree.insert(node.id.clone(), node);
				}
			},
			ChangeItems::DelNode(nids) => {
				for nid in nids {
					self.tree.remove(&nid);
				}
			},
			ChangeItems::Disconnect(nids) => {
				for nid in nids {
					let mnode = self.tree.remove(&nid);
					if let Some(node) = mnode {
						self.disconnected.insert(nid, node);
					}
				}
			},
			ChangeItems::Connect(nids) => {
				for nid in nids {
					let mnode = self.disconnected.remove(&nid);
					if let Some(node) = mnode {
						self.tree.insert(nid, node);
					}
				}
			},
		}
	}
	fn no_disconnected_children(&self) -> bool {
		self.tree.values()
			.all(|node| {
					node.children.iter().all(|nid| self.disconnected.get(nid).is_some())
			})
	}
}

#[derive(Debug)]
pub struct Node {
	pub id: NodeId,
	pub parent: NodeId,
	pub children: Vec<NodeId>,
	pub text: String,
	pub role: Role,
}

fn change_item() -> impl Strategy<Value = ChangeItems> {
	prop_oneof![
		proptest::collection::vec(node(), 0..100).prop_map(ChangeItems::AddNode),
		proptest::collection::vec(any::<u32>().prop_map(NodeId), 0..100).prop_map(ChangeItems::DelNode),
		proptest::collection::vec(any::<u32>().prop_map(NodeId), 0..100).prop_map(ChangeItems::Disconnect),
		proptest::collection::vec(any::<u32>().prop_map(NodeId), 0..100).prop_map(ChangeItems::Connect),
	]
}

#[derive(Debug)]
pub enum ChangeItems {
	AddNode(Vec<Node>),
	DelNode(Vec<NodeId>),
	Disconnect(Vec<NodeId>),
	Connect(Vec<NodeId>),
}

proptest! {
	#[test]
	fn test_state_transitions(
		node_changes in proptest::collection::vec(
			change_item(),
			0..50
		)
	) {
		let mut tree = NodeMap::default();
		assert!(tree.no_disconnected_children());
		for change in node_changes {
			tree.apply(change);
			assert!(tree.no_disconnected_children());
		}
	}
}
