pub mod interpreter;
pub mod compiler;


type NodeOffset = usize;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MatchResult {
  Reject = 0,
  Accept = 1,
}

impl Into<bool> for MatchResult {
  fn into(self) -> bool {
    match self {
        Reject => false,
        Accept => true,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetNodeOffset {
  Next(NodeOffset),
  Result(MatchResult),
}


/// ## Edge
/// if src[offset] == char { goto next_status }
#[derive(Debug)]
pub struct Edge (pub u8, pub NodeOffset, pub TargetNodeOffset);


/// ## Node
/// one node brench fork as follow
/// if offset == length { goto unaccept; }
/// for Edge, ...
/// goto unaccept;
pub type Node = Vec<Edge>;


/// ## Status Transform Graph (STG)
#[derive(Debug, Default)]
pub struct StatusTransGraph (pub Vec<Node>);


impl StatusTransGraph {
  pub fn new(i: Vec<Node>) -> Self {
    Self(i)
  }

  pub fn add_edge(&mut self, i: Edge) {
    self.0[i.1].push(i)
  }
}
