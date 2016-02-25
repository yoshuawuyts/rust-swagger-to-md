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
pub fn swagger_to_md (inp: &str, opts: Options) -> Option<&str> {
  let json: Value = serde_json::from_str(inp).unwrap();

  if opts.yaml {
    println!("--yaml is not implemented yet, careful");
  }

  let rows = match collect_values(&json) {
    Some(m) => m,
    None => return None
  };

  for row in rows {
    println!("{} {} {}", row.method, row.path, row.summary);
  }

  // println!("{:?}", matches);
  let tmp: &'static str = "hey";
  return Some(tmp);
}

// extract values from JSON struct
// .paths[http_method].summary
fn collect_values <'a> (json: &Value) -> Option<Vec<Swagger_value>> {
  let mut vec = Vec::new();
  let raw_paths = json.lookup("paths").unwrap();

  let paths = match raw_paths.as_object() {
    Some(path) => path,
    None => return None,
  };

  for (path, raw_data) in paths.iter() {
    let data = match raw_data.as_object() {
      Some(path) => path,
      None => return None,
    };

    for (method, raw_data) in data.iter() {
      let dft: &'static str = "";

      let summary = match raw_data.lookup("description") {
        Some(s) => match s.as_string() {
          Some(s) => s,
          None => dft,
        },
        None => dft,
      };

      let val = Swagger_value {
        method: &method,
        path: &path,
        summary: &summary,
      };

      vec.push(val);
    }
  }

  return Some(vec);
}
