pub mod dfa;
pub mod nfa;

type NodeOffset = usize;


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum MatchResult {
  Reject = 0,
  Accept = 1,
}

impl From<MatchResult> for bool {
  fn from(value: MatchResult) -> Self {
    match value {
      MatchResult::Reject => false,
      MatchResult::Accept => true,
    }
  }
}

impl MatchResult {
  pub fn map<T>(&self, f: impl Fn() -> T) -> Option<T> {
    match self {
      MatchResult::Reject => None,
      MatchResult::Accept => Some(f()),
    }
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetNodeOffset {
  Next(NodeOffset),
  Result(MatchResult),
}

impl TargetNodeOffset {
  pub fn to_node(&self) -> Option<NodeOffset> {
    if let Self::Next(r) = self {
      return Some(*r);
    }
    None
  }
}

/// ## Status Transform Graph (STG)
#[derive(Debug, Default)]
pub struct StatusTransGraph<T>(pub Vec<T>);


impl<T> StatusTransGraph<T> {
  pub fn new(i: Vec<T>)-> Self {
    Self(i)
  }

  pub fn add_edge(&mut self, i: T) {
    self.0.push(i)
  }
}