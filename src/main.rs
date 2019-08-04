extern crate argparse;
extern crate rust_util;

mod opt;
mod base58;

use std::{
    fs::File,
    io::{
        self,
        prelude::*,
    }
};
use base58::{ToBase58, FromBase58};

use opt::*;
use rust_util::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");

fn print_version() {
    print!(r#"base58 {} - {}
Copyright (C) 2019 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>

Written by Hatter Jiang
"#, VERSION, &GIT_HASH[0..7]);
}

fn encode_base58(read: &mut Read, new_line: bool) {
    let mut buffer = Vec::new();
    match read.read_to_end(&mut buffer) {
        Err(err) => {
            print_message(MessageType::ERROR, &format!("Read from stdin failed: {}", err));
            return;
        },
        Ok(_) => (),
    };
    print!("{}{}", &buffer.to_base58(), match new_line { false => "", true => "\n", });
}

fn decode_base58(read: &mut Read, token: &str, new_line: bool, verbose: bool) {
    let mut buffer = String::new();
    match read.read_to_string(&mut buffer) {
        Err(err) => {
            print_message(MessageType::ERROR, &format!("Read {} failed: {}", token, err));
            return;
        },
        Ok(_) => (),
    };
    if verbose {
        print_message(MessageType::INFO, &format!("Read content: {}", &buffer));
    }
    match buffer.as_str().trim().from_base58() {
        Err(err) => {
            print_message(MessageType::ERROR, &format!("Decode base58 from {}, failed: {:?}", token, err));
            return;
        },
        Ok(bs) => {
            io::stdout().write(bs.as_slice()).unwrap();
            if new_line {
                println!();
            } else {
                io::stdout().flush().unwrap();
            }
        },
    };
}



fn main() {
    let options = Options::new_and_parse_args();
    
    if options.version {
        print_version();
        return;
    }

    if options.file.len() > 0 {
        let mut f = match File::open(&options.file) {
            Err(err) => {
                print_message(MessageType::ERROR, &format!("Open file: {}, failed: {}", &options.file, err));
                return;
            },
            Ok(f) => f,
        };
        if options.decode {
            decode_base58(&mut f, &format!("file: {}", &options.file), options.new_line, options.verbose);
        } else {
            encode_base58(&mut f, options.new_line);
        }
    } else {
        if options.decode {
            decode_base58(&mut io::stdin(), "stdin", options.new_line, options.verbose);
        } else {
            encode_base58(&mut io::stdin(), options.new_line);
        }
    }
}

