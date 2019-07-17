
fn print_usage() {
    print!(r#"Usage: base58 [OPTION]... [FILE]
Base58 encode or decode FILE, or standard input, to standard output.

With no FILE, or when FILE is -, read standard input.
"#);
}

fn main() {
    print_usage();
    println!("hello world, base58");
}

