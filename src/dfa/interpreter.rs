use super::TargetNodeOffset::*;
use super::*;

///# Safety
/// It is safety if and only if the input is nfa2dfa
#[inline]
pub unsafe fn raw_interpret(
  _bytecodes: *const [Node],
  _src: *const u8,
  _length: usize,
) -> Option<usize> {
  // let bytes = str;
  // let bytes = str.as_bytes();
  // raw_interpret(bytecodes, bytes.as_ptr(), bytes.len())
  todo!()
}


///# Safety
/// It is safe if and only if the input is nfa2dfa
#[inline]
pub unsafe fn interpret(bytecodes: &[Node], src: *const u8, length: usize) -> Option<usize> {
  let mut pc = 0;
  let mut offset = 0;
  'node: loop {
    if offset == length {
      return None;
    };
    // fetch
    let node = bytecodes.get_unchecked(pc);
    // match one level (all branch)
    for i in node {
      if *src.add(offset) == i.0 {
        match i.2 {
          Next(next) => {
            pc = next;
            offset += 1;
            continue 'node;
          }
          Result(r) => return r.map(|| offset),
        }
      };
    }
    return None;
  }
}
