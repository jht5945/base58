extern crate argparse;
#[macro_use]
extern crate rust_util;

mod opt;
mod base58;

use std::{
    fs::File,
    io::{ self, prelude::* },
};
use base58::{ ToBase58, FromBase58 };
use opt::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");
const BUILD_DATE: &str = env!("BUILD_DATE");

fn print_version(options: &Options) {
    print!(r#"base58 {} - {}
Copyright (C) 2019-2020 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>

Written by Hatter Jiang
"#, VERSION, &GIT_HASH[0..7]);
    if options.verbose {
        debugging!("Full GIT_HASH: {}", GIT_HASH);
        debugging!("Build date: {}", BUILD_DATE);
    }
}

fn encode_base58(read: &mut dyn Read, options: &Options) {
    let mut buffer = Vec::with_capacity(1024);
    if options.verbose {
        debugging!("Start read input.");
    }
    if let Err(err) = read.read_to_end(&mut buffer) {
        failure!("Read from stdin failed: {}", err);
        return;
    }
    if options.verbose {
        debugging!("Read input finished.");
    }
    print!("{}{}", &buffer.to_base58(), iff!(options.new_line, "\n", ""));
}

fn decode_base58(read: &mut dyn Read, token: &str, options: &Options) {
    let mut buffer = String::with_capacity(1024);
    if let Err(err) = read.read_to_string(&mut buffer) {
        failure!("Read {} failed: {}", token, err);
        return;
    }
    if options.verbose {
        failure!("Read content: {}", &buffer);
    }
    match buffer.as_str().trim().from_base58() {
        Err(err) => failure!("Decode base58 from {}, failed: {:?}", token, err),
        Ok(bs) => {
            io::stdout().write(bs.as_slice()).ok();
            if options.new_line {
                println!();
            } else {
                io::stdout().flush().ok();
            }
        },
    }
}

fn get_read_in(file: &str) -> Option<Box<dyn Read>> {
    if file.is_empty() {
        Some(Box::new(io::stdin()))
    } else {
        match File::open(file) {
            Ok(f) => Some(Box::new(f)), Err(err) => {
                failure!("Open file: {}, failed: {}", file, err);
                None
            },
        }
    }
}


fn main() {
    let options = Options::new_and_parse_args();
    
    if options.version {
        print_version(&options);
        return;
    }

    let mut read_in  = match get_read_in(&options.file) {
        Some(r) => r, None => return,
    };
    let hint = iff!(options.file.is_empty(), "stdin".to_owned(), format!("file: {}", &options.file));

    if options.decode {
        decode_base58(&mut read_in, &hint, &options);
    } else {
        encode_base58(&mut read_in, &options);
    }
}

