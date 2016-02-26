#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde_json;
extern crate serde;

use serde_json::Value;

// define options, exposed so [[bin]]
// knows which opts to pass
pub struct Options {
  pub yaml: bool,
}

// disassembled JSON values
struct SwaggerValue<'a> {
  method: &'a str,
  path: &'a str,
  summary: &'a str,
}

// transform swagger into markdown
pub fn swagger_to_md<'a> (inp: &str, opts: Options) -> Option<String> {
  let json: Value = serde_json::from_str(inp).unwrap();

  if opts.yaml {
    println!("--yaml is not implemented yet, careful");
  }

  let rows = match collect_values(&json) {
    Some(m) => m,
    None => return None
  };

  let mut res = String::new();
  res.push_str("<table>\n");
  res.push_str("<tr>\n");
  res.push_str("<td><b>Path</b></td>\n");
  res.push_str("<td><b>Method</b></td>\n");
  res.push_str("<td><b>Summary</b></td>\n");
  res.push_str("</tr>\n");
  for row in rows {
    res.push_str("<tr>\n");
    res.push_str("<td>");
    res.push_str(&row.path);
    res.push_str("</td>\n");
    res.push_str("<td>");
    res.push_str(&row.method.to_uppercase());
    res.push_str("</td>\n");
    res.push_str("<td>");
    res.push_str(&row.summary);
    res.push_str("</td>\n");
    res.push_str("</tr>\n");
  }
  res.push_str("</table>\n");

  return Some(res);
}

// extract values from JSON struct
// .paths[http_method].summary
fn collect_values <'a> (json: &Value) -> Option<Vec<SwaggerValue>> {
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
      let dft = "";

      let summary = match raw_data.lookup("description") {
        Some(s) => match s.as_string() {
          Some(s) => s,
          None => dft,
        },
        None => dft,
      };

      let val = SwaggerValue {
        method: &method,
        path: &path,
        summary: &summary,
      };

      vec.push(val);
    }
  }

  return Some(vec);
}
