extern crate term;
extern crate argparse;

mod base58;

use std::io::{
    self,
    prelude::*,
};
use std::fs::File;
use argparse::{ArgumentParser, StoreTrue, Store};
use base58::{ToBase58, FromBase58};

const VERSION: &str = "0.1";

fn print_version() {
    print!(r#"base58 {}
Copyright (C) 2019 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>

Written by Hatter Jiang
"#, VERSION);
}

pub enum MessageType {
    INFO, OK, WARN, ERROR
}

fn print_message(mt: MessageType, message: &str) {
    let mut t = term::stdout().unwrap();
    match mt {
        MessageType::OK => {
            t.fg(term::color::GREEN).unwrap();
            write!(t, "[OK   ]").unwrap();
        },
        MessageType::WARN => {
            t.fg(term::color::YELLOW).unwrap();
            write!(t, "[WARN ]").unwrap();
        },
        MessageType::ERROR => {
            t.fg(term::color::RED).unwrap();
            write!(t, "[ERROR]").unwrap();
        },
        MessageType::INFO => {
            write!(t, "[INFO ]").unwrap();
        }
    }
    t.reset().unwrap();
    println!(" {}", message);
}

fn encode_base58(read: &mut Read, n: bool) {
    let mut buffer = Vec::new();
    match read.read_to_end(&mut buffer) {
        Err(err) => {
            print_message(MessageType::ERROR, &format!("Read from stdin failed: {}", err));
            return;
        },
        Ok(_) => (),
    };
    print!("{}{}", &buffer.to_base58(), match n { true => "", false => "\n", });
}

// !FIXME decode base59 always fail.
fn decode_base58(read: &mut Read, token: &str, n: bool, verbose: bool) {
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
            if ! n {
                println!();
            } else {
                io::stdout().flush().unwrap();
            }
        },
    };
}


fn main() {
    let mut n = false;
    let mut verbose = false;
    let mut decode = false;
    let mut version = false;
    let mut file = String::new();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("base58 - command line base58 convert tool.");
        ap.refer(&mut decode).add_option(&["-d", "--decode"], StoreTrue, "Decode data");
        ap.refer(&mut n).add_option(&["-n"], StoreTrue, "Do not output the trailing newline");
        ap.refer(&mut version).add_option(&["-v", "--version"], StoreTrue, "Print version");
        ap.refer(&mut verbose).add_option(&["--verbose"], StoreTrue, "Verbose output");
        ap.refer(&mut file).add_argument("FILE", Store, "FILE");
        ap.parse_args_or_exit();
    }
    
    if version {
        print_version();
        return;
    }

    if file.len() > 0 {
        let mut f = match File::open(&file) {
            Err(err) => {
                print_message(MessageType::ERROR, &format!("Open file: {}, failed: {}", &file, err));
                return;
            },
            Ok(f) => f,
        };
        if decode {
            decode_base58(&mut f, &format!("file: {}", &file), n, verbose);
        } else {
            encode_base58(&mut f, n);
        }
    } else {
        if decode {
            decode_base58(&mut io::stdin(), "stdin", n, verbose);
        } else {
            encode_base58(&mut io::stdin(), n);
        }
    }
}

