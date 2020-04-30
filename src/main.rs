extern crate argparse;
extern crate rust_util;

mod opt;
mod base58;

use std::{
    fs::File,
    io::{ self, prelude::*, },
};
use base58::{ ToBase58, FromBase58, };

use opt::*;
use rust_util::{ iff, util_msg::*, };

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
        print_message(MessageType::DEBUG, &format!("Full GIT_HASH: {}", GIT_HASH));
        print_message(MessageType::DEBUG, &format!("Build date: {}", BUILD_DATE));
    }
}

fn encode_base58(read: &mut dyn Read, options: &Options) {
    let mut buffer = Vec::with_capacity(1024);
    if options.verbose {
        print_message(MessageType::DEBUG, "Start read input.");
    }
    if let Err(err) = read.read_to_end(&mut buffer) {
        print_message(MessageType::ERROR, &format!("Read from stdin failed: {}", err));
        return;
    }
    if options.verbose {
        print_message(MessageType::DEBUG, "Read input finished.");
    }
    print!("{}{}", &buffer.to_base58(), match options.new_line { false => "", true => "\n", });
}

fn decode_base58(read: &mut dyn Read, token: &str, options: &Options) {
    let mut buffer = String::with_capacity(1024);
    if let Err(err) = read.read_to_string(&mut buffer) {
        print_message(MessageType::ERROR, &format!("Read {} failed: {}", token, err));
        return;
    }
    if options.verbose {
        print_message(MessageType::INFO, &format!("Read content: {}", &buffer));
    }
    match buffer.as_str().trim().from_base58() {
        Err(err) => print_message(MessageType::ERROR, &format!("Decode base58 from {}, failed: {:?}", token, err)),
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


fn main() {
    let options = Options::new_and_parse_args();
    
    if options.version {
        print_version(&options);
        return;
    }

    let mut read_in: Box<dyn Read> = if options.file.is_empty() {
        Box::new(io::stdin())
    } else {
        match File::open(&options.file) {
            Ok(f) => Box::new(f), Err(err) => {
                print_message(MessageType::ERROR, &format!("Open file: {}, failed: {}", &options.file, err));
                return;
            },
        }
    };
    let hint = iff!(options.file.is_empty(), "stdin".to_owned(), format!("file: {}", &options.file));

    if options.decode {
        decode_base58(&mut read_in, &hint, &options);
    } else {
        encode_base58(&mut read_in, &options);
    }
}

