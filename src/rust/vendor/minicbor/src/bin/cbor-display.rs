use std::env;
use std::fs;
use std::process;
use std::io::{self, Read};

const SUMMARY: &str = r#"Usage: cbor-display OPTION*

With no OPTION or if OPTION is -, input is read from stdin.

  -f | --file PATH  Display contents of the file at PATH.
  -h | --help       Show this help message."#;


fn main() {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("-f") | Some("--file") =>
            if let Some(p) = args.next() {
                match fs::read(&p) {
                    Ok(f)  => println!("{}", minicbor::display(&f)),
                    Err(e) => {
                        eprintln!("Failed to read \"{}\": {}.", p, e);
                        process::exit(2)
                    }
                }
            } else {
                eprintln!("-f | --file requires a path as argument.");
                process::exit(1)
            }
        Some("-") | None => {
            let mut v = Vec::new();
            match io::stdin().read_to_end(&mut v) {
                Ok(_)  => println!("{}", minicbor::display(&v)),
                Err(e) => {
                    eprintln!("Failed to read from stdin: {}.", e);
                    process::exit(3)
                }
            }
        }
        Some("-h") | Some("--help") => {
            println!("{}", SUMMARY)
        }
        Some(unknown) => {
            eprintln!("Unknown option: {}\n\n{}", unknown, SUMMARY);
            process::exit(1)
        }
    }
}
