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
  method: String,
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
  let header = SwaggerValue {
    method: "<b>Method</b>".into(),
    path: "<b>Path</b>",
    summary: "<b>Summary</b>",
  };
  res.push_str(&format_string(&header));

  for row in rows {
    res.push_str(&format_string(&row));
  }
  res.push_str("</table>\n");

  return Some(res);
}

fn format_string<'a> (row: &SwaggerValue) -> String {
  let mut res = String::new();
  res.push_str("<tr>\n");
  res.push_str(&format!("<td>{}</td>\n", row.path));
  res.push_str(&format!("<td>{}</td>\n", row.method));
  res.push_str(&format!("<td>{}</td>\n", row.summary));
  res.push_str("</tr>\n");
  return res;
}

// extract values from JSON struct
// .paths[http_method].summary
fn collect_values<'a> (json: &'a Value) -> Option<Vec<SwaggerValue<'a>>> {
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

      let uc_method: String = method.to_uppercase();
      let val = SwaggerValue {
        method: uc_method,
        path: &path,
        summary: &summary,
      };

      vec.push(val);
    }
  }

  return Some(vec);
}
