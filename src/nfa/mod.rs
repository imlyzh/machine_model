mod todfa;
use crate::{NodeOffset, TargetNodeOffset, StatusTransGraph};


pub struct Node(pub u8, pub NodeOffset, pub Vec<TargetNodeOffset>);

pub type NFAStatusTransGraph = StatusTransGraph<Node>;

