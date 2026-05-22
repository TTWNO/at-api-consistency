use std::collections::HashMap;
use atspi::{Role, connection::set_session_accessibility,
	Event,
	events::ObjectEvents,
	events::CacheEvents,
};
use std::error::Error;
use tokio_stream::StreamExt;

#[derive(Debug)]
pub enum ChangeItems {
	AddNode(Vec<Node>),
	DelNode(Vec<NodeId>),
	Disconnect(Vec<NodeId>),
	Connect(Vec<NodeId>),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct NodeId(String);

pub struct NodeMap {
	tree: HashMap<NodeId, Node>,
	disconnected: HashMap<NodeId, Node>,
}

#[derive(Debug)]
pub struct Node {
	pub id: NodeId,
	pub parent: NodeId,
	pub children: Vec<NodeId>,
	pub text: String,
	pub role: Role,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let _ = set_session_accessibility(true).await?;
	let atspi = atspi::AccessibilityConnection::new().await?;
	atspi.register_event::<ObjectEvents>().await?;
	atspi.register_event::<CacheEvents>().await?;
	let con = atspi.connection();

	let events = atspi.event_stream();
	tokio::pin!(events);

	while let Some(Ok(ev)) = events.next().await {
		let change = match ev {
			Event::Cache(CacheEvents::Add(add)) => {
				//let apply = ChangeItems::Add(vec![Node {
				//	id: add.node_added.id.path(),
				//	parent: add.node_added.parent.path(),
				//	children: Vec::new(),
				//	/*children: add.node_added.children.iter()
				//			.map(|ch| ch.path()).collect(),
				//	*/
				//	text: String::new(),
				//	role: add.node_added.role,
				//}]);
				println!("{add:?}");
			},
			Event::Cache(CacheEvents::Remove(rem)) => {
				println!("{rem:?}");
			},
			Event::Object(ObjectEvents::ChildrenChanged(ch)) => {
				println!("{ch:?}");
			},
			_ => {},
		};
/*
		if ev.item().is_null() {
			continue;
		}
		let acc = match change.item.as_accessible_proxy(con).await {
			Ok(a) => a,
			Err(e) => {
				println!("(skipping; cannot build accessible proxy: {e})");
				continue;
			}
		};
		println!(
			"\n=== {} {} ===",
			change.item.name_as_str().unwrap_or("?"),
			change.item.path_as_str()
		);
		*/
	}
	Ok(())
}
