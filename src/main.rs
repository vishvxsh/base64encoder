mod base64;

use std::env;
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt; // provides OsString::into_vec() for raw-byte CLI args
use std::io::{self, Read, Write};

fn print_usage() {
    eprintln!("Usage: base64encoder <encode|decode> [string]");
    eprintln!("If no string is provided, reads from standard input.");
}

fn main() {
    if let Err(e) = run() {
        if let Some(io_err) = e.downcast_ref::<io::Error>() {
            if io_err.kind() == io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
        }
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<OsString> = env::args_os().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    if args.len() > 3 {
        return Err("Too many arguments".into());
    }
    
    let command = args[1].to_string_lossy();
    
    let stdin = io::stdin();
    let mut stdin_lock;
    let mut cursor_reader;
    let mut input: &mut dyn Read = if args.len() == 3 {
        cursor_reader = io::Cursor::new(args[2].clone().into_vec());
        &mut cursor_reader
    } else {
        stdin_lock = stdin.lock();
        &mut stdin_lock
    };
    
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());
    
    match command.as_ref() {
        "encode" => {
            base64::encode(&mut input, &mut output)?;
            output.write_all(b"\n")?;
        }
        "decode" => {
            base64::decode(&mut input, &mut output)?;
            output.write_all(b"\n")?;
        }
        _ => {
            return Err(format!("Unknown command: {}", command).into());
        }
    }
    
    output.flush()?;
    
    Ok(())
}
