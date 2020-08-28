extern crate libc;

struct z80context;

extern "C" {
  fn Z80RESET(input: z80context) -> libc::c_void;
}

fn main() {
  
}