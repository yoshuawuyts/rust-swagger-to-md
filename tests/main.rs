extern crate swagger_to_md;

use std::io::BufReader;
use std::fs::File;
use std::io::Read;

use swagger_to_md::Options as lib_Options;
use swagger_to_md::swagger_to_md as lib;

#[test]
fn parses_stuff() {
  let lib_options = lib_Options {
    yaml: false,
  };

  let f = File::open("./fixtures/api.json").unwrap();
  let mut rs = BufReader::new(f);
  let mut file = String::new();
  rs.read_to_string(&mut file).unwrap();

  match lib(&file, lib_options) {
    Some(m) => println!("{}", m),
    None => println!("noooope"),
  }
}
