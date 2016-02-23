#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use serde_json::Value;

// define options, exposed so [[bin]]
// knows which opts to pass
pub struct Options {
  pub yaml: bool,
}

// transform swagger into markdown
pub fn swagger_to_md (inp: &str, opts: Options) {
  let deserialized: Value = serde_json::from_str(inp).unwrap();
  println!("{:?}", deserialized);
}
