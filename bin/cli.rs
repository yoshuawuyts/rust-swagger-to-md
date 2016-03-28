extern crate swagger_to_md;
extern crate getopts;

use std::io::BufReader;
use getopts::Options;
use std::fs::File;
use std::io::Read;
use std::env;
use std::io;

use swagger_to_md::Options as lib_Options;
use swagger_to_md::swagger_to_md as lib;

// parse cli args and run lib
fn main () {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut options = Options::new();
  options.optflag("h", "help", "Output usage information");
  options.optflag("y", "yaml", "Parse spec as YAML");

  let matches = match options.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  // help command
  if matches.opt_present("h") {
    print_usage(&program, options);
    return;
  }

  // parse options
  let mut lib_options = lib_Options {
    yaml: false,
  };

  if matches.opt_present("y") {
    lib_options.yaml = true;
  }

  // call lib with args and opts
  if !matches.free.is_empty() {
    // parse cli arg
    let input = matches.free.clone();

    // todo: handle fd error explicitely
    let f = File::open(&input[0]).unwrap();
    let mut rs = BufReader::new(f);
    let mut file = String::new();
    rs.read_to_string(&mut file).unwrap();

    match lib(&file, lib_options) {
      Ok(v) => match v {
        Some(m) => println!("{}", m),
        None => println!("No data found"),
      },
      Err(e) => println!("Error found {}", e)
    }
  } else {
    // parse stdin
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(_) => match lib(&input, lib_options) {
        Ok(v) => match v {
          Some(m) => println!("{}", m),
          None => println!("No data found"),
        },
        Err(e) => println!("Error found {}", e)
      },
      Err(e) => println!("Error found {}", e),
    }
  };
}

// print CLI usage
fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} <file> [options]", program);
  print!("{}", opts.usage(&brief));
}
