use crate::{NodeOffset, TargetNodeOffset, StatusTransGraph};
pub mod interpreter;
pub mod compiler;



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
pub type DFAStatusTransGraph = StatusTransGraph<Node>;
