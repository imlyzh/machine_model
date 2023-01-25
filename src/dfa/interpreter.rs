use super::*;
use super::TargetNodeOffset::*;


#[inline]
pub unsafe fn interpret<'a>(bytecodes: &'a [Node], str: &str) -> bool {
  // let bytes = str;
  let bytes = str.as_bytes();
  raw_interpret(bytecodes, bytes.as_ptr(), bytes.len())
}


#[inline]
pub unsafe fn raw_interpret<'a>(bytecodes: &'a [Node], src: *const u8, length: usize) -> bool {
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
