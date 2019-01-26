#[macro_use]
extern crate clap;

use clap::{App, Arg};

// Supported input formats
arg_enum! {
    #[derive(PartialEq)]
    pub enum InputFormat {
        DRONE
    }
}

// Supported output formats
arg_enum! {
    #[derive(PartialEq)]
    pub enum OutputFormat {
        SVG
    }
}

fn main() {
    let _matches = App::new("CIV - CI Pipeline Viewer")
        .version("0.1")
        .author("Giedrius Statkevičius <giedriuswork@gmail.com>")
        .about("Render CI pipeline files into graphical images")
        .arg(
            Arg::with_name("INPUT")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUTFMT")
                .short("f")
                .long("ifmt")
                .value_name("FORMAT")
                .help("Sets the input format to use")
                .possible_values(&InputFormat::variants())
                .required(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets the output file to use")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUTFMT")
                .short("m")
                .long("ofmt")
                .value_name("FORMAT")
                .help("Sets the output format to use")
                .possible_values(&OutputFormat::variants())
                .required(true),
        )
        .get_matches();
    println!("hey")
}
