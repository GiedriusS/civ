extern crate clap;

use clap::{Arg, App};

// Supported input formats
enum InputFormat {
    DRONE
}

// Supported output formats
enum OutputFormat {
    SVG
}

fn main() {
    let _matches = App::new("CIV - CI Pipeline Viewer")
                        .version("0.1")
                        .author("Giedrius Statkevičius <giedriuswork@gmail.com>")
                        .about("Render CI pipeline files into graphical images")
                        .arg(Arg::with_name("INPUT")
                            .short("i")
                            .long("input")
                            .value_name("FILE")
                            .help("Sets the input file to use")
                            .required(true)
                            .takes_value(true))
                        .arg(Arg::with_name("INPUTFMT")
                            .short("f")
                            .long("ifmt")
                            .value_name("FORMAT")
                            .help("Sets the input format to use")
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::with_name("v")
                            .short("v")
                            .multiple(true)
                            .help("Sets the level of verbosity"))
                        .get_matches();
    println!("hey")
}
