#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde_json;
extern crate serde;

use serde_json::Value;
use std::str;

// define options, exposed so [[bin]]
// knows which opts to pass
pub struct Options {
  pub yaml: bool,
}

// disassembled JSON values
struct Swagger_value<'a> {
  method: &'a str,
  path: &'a str,
  summary: &'a str,
}

// transform swagger into markdown
pub fn swagger_to_md (inp: &str, opts: Options) {
  let json: Value = serde_json::from_str(inp).unwrap();

  if opts.yaml {
    println!("--yaml is not implemented yet, careful");
  }

  collect_values(&json).unwrap();
}

// extract values from JSON struct
// .paths[http_method].summary
fn collect_values (json: &Value) -> Option<Vec<Swagger_value>> {
  let mut vec = Vec::new();
  let raw_paths = json.lookup("paths").unwrap();

  // path
  if let Some(paths) = raw_paths.as_object() {
    for (path, raw_data) in paths.iter() {

      // keys
      if let Some(data) = raw_data.as_object() {
        for (method, raw_data) in data.iter() {
          let desc = match raw_data.find_path("description") {
            Some(s) => s,
            None => str::from_utf8("").unwrap(),
          };
          println!("{:?}", desc);
        }
      } else {
        return None;
      }
    }
  } else {
    return None;
  }

  return Some(vec);
}
