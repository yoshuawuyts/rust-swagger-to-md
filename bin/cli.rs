extern crate swagger_to_md;
extern crate getopts;

use swagger_to_md::swagger_to_md as lib;
use swagger_to_md::Options as lib_Options;
use getopts::Options;
use std::env;

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

  let mut lib_options = lib_Options {
    yaml: false,
  };

  if matches.opt_present("y") {
    lib_options.yaml = true;
  }

  if matches.opt_present("h") {
    print_usage(&program, options);
  } else {
    lib(&program, lib_options);
  }
}

// print CLI usage
fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} <file> [options]", program);
  print!("{}", opts.usage(&brief));
}
