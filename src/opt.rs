use argparse::{ArgumentParser, StoreTrue, Store};

pub struct Options {
    pub version: bool,
    pub verbose: bool,
    pub decode: bool,
    pub new_line: bool,
    pub file: String,
}

impl Options {
    pub fn new() -> Options {
        Options {
            version: false,
            verbose: false,
            decode: false,
            new_line: false,
            file: String::new(),
        }
    }

    pub fn new_and_parse_args() -> Options {
        let mut options = Options::new();
        {
            let mut ap = ArgumentParser::new();
            ap.set_description("base58 - command line base58 convert tool.");
            ap.refer(&mut options.decode).add_option(&["-d", "--decode"], StoreTrue, "Decode data");
            ap.refer(&mut options.new_line).add_option(&["--new-line"], StoreTrue, "Do output the trailing newline");
            ap.refer(&mut options.version).add_option(&["-v", "--version"], StoreTrue, "Print version");
            ap.refer(&mut options.verbose).add_option(&["--verbose"], StoreTrue, "Verbose output");
            ap.refer(&mut options.file).add_argument("FILE", Store, "FILE");
            ap.parse_args_or_exit();
        }

        options
    }
}
