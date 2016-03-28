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
pub fn swagger_to_md<'a>(inp: &str, opts: Options)
  -> Result<Option<String>, &'static str>
{
  let json: Value = match serde_json::from_str(inp) {
    Ok(v) => v,
    Err(_) => return Err("Could not parse JSON"),
  };

  if opts.yaml {
    return Err("--yaml flag is not implemented");
  }

  let rows = match collect_values(&json) {
    Ok(v) => match v {
      Some(m) => m,
      None => return Ok(None)
    },
    Err(e) => return Err(e)
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

  return Ok(Some(res));
}

// format Swagger into HTML tables
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
fn collect_values<'a> (json: &'a Value) ->
  Result<Option<Vec<SwaggerValue<'a>>>, &'static str>
{
  let mut vec = Vec::new();
  let raw_paths = match json.lookup("paths") {
    Some(v) => v,
    None => return Ok(None),
  };

  let paths = match raw_paths.as_object() {
    Some(v) => v,
    None => return Ok(None),
  };

  for (path, raw_data) in paths.iter() {
    let data = match raw_data.as_object() {
      Some(v) => v,
      None => return Ok(None),
    };

    for (method, raw_data) in data.iter() {
      let dft = "";

      let summary = match raw_data.lookup("description") {
        Some(v) => match v.as_string() {
          Some(v) => v,
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

  return Ok(Some(vec));
}
