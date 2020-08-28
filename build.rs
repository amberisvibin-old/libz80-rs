fn main() {

  //link z80 lib
  println!("cargo:rustc-link-lib=z80");


  cc::Build::new()
    .file("libz80/z80.c")
    .include("libz80/")
    .compile("z80");
}