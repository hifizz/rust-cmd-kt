extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;
use std::io::{ Write };

fn main() {
    let _matches = App::new("kt")
        .version("0.1.0")
        .author("zilin fizzstack@gmail.com")
        .about("A tool like cat in rust")
        .arg(Arg::with_name("FILE"))
        .help("File to print to stdout")
        .get_matches();

    if let Some(file) = _matches.value_of("FILE") {
        println!("Value for file argument: {}", file);
        if Path::new(&file).exists() {
            match File::open(file) {
                Ok(mut f) => {
                    let mut data = String::new();
                    f.read_to_string(&mut data)
                        .expect("kt error: unable to read file");
                    let stdout = std::io::stdout();
                    let mut handle = std::io::BufWriter::new(stdout);
                    match writeln!(handle, "{}", data) {
                        Ok(_res) => {},
                        Err(err) => {
                            eprintln!("[kt Error] Unable to display the file contents. {:?}", err);
                            process::exit(1);
                        },
                    }

                }
                Err(err) => {
                    eprintln!("[kt error]: unable to read the file. {:?}", err);
                    process::exit(1);
                }
            }
            // println!("File exist!!");
            // let mut f = File::open(file).expect("[kt error] File not found");
            // let mut data = String::new();
            // f.read_to_string(&mut data).expect("[kt error] unable to read the file");
            // println!("{}", data);
        } else {
            eprintln!("[kt Error] no such file or directory.");
            process::exit(1);
        }
    }
}

// fn main() {
//     println!("Hello, world!");
// }
