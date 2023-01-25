use super::*;
use super::TargetNodeOffset::*;

#[derive(Debug, Clone)]
pub struct Interpreter<'a> {
  pub bytecodes: &'a [Node],
  pub pc: usize,
  pub len: usize,
  pub str: *const [u8],
}

// impl Interpreter<'a> {
  // pub fn new(bcs: &'a [Node], ) {
  // }
// }

#[inline]
pub unsafe fn interpreter<'a>(bytecodes: &'a [Node], str: &str) -> bool {
  // let bytes = str;
  let bytes = str.as_bytes();
  raw_interpreter(bytecodes, bytes.as_ptr(), bytes.len())
}

#[inline]
pub unsafe fn raw_interpreter<'a>(bytecodes: &'a [Node], src: *const u8, length: usize) -> bool {
  let mut pc = 0;
  let mut offset = 0;
  loop {
    if offset == length { return false };
    // fetch
    let node = bytecodes.get_unchecked(pc);
    for i in node {
      if *src.offset(offset as isize) == i.0 {
        match i.2 {
          Next(next) => { pc = next; offset += 1; continue; },
          Result(r) => return r.into(),
        }
      };
    }
    return false;
  }
}
