

/// ## Node
/// one node brench fork as follow
/// if offset == length { goto unaccept; }
/// Edge, ...
// #[derive(Debug, Default)]
// pub struct Node(pub Vec<Edge>);
pub type Node = Vec<Edge>;
pub type NodeOffset = usize;


/// ## Edge
///   if src[offset] == matching { goto next_status }
#[derive(Debug, Default)]
pub struct Edge {
  pub status: NodeOffset,
  pub next_status: NodeOffset,
  pub matching: char,
}


/// ## Status Transform Graph (STG)
#[derive(Debug, Default)]
pub struct StatusTransGraph {
  pub nodes: Vec<Node>,
  acceptable_node: Node,
  unacceptable_node: Node,
}


impl StatusTransGraph {
  pub fn new(acceptable_node: Node, unacceptable_node: Node) -> Self {
    Self {
        nodes: vec![],
        acceptable_node,
        unacceptable_node,
    }
  }

  pub fn from(nodes: Vec<Node>, acceptable_node: Node, unacceptable_node: Node) -> Self {
    Self {
        nodes,
        acceptable_node,
        unacceptable_node,
    }
  }

  pub fn add_edge(&mut self, status: usize, next_status: usize, matching: char) {
    self.nodes[status].push(Edge{status, next_status, matching})
  }
}
