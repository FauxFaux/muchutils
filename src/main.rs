use std::env::ArgsOs;
use std::ffi::OsStr;
use std::path::Path;

use anyhow::Result;

mod count_bytes;
mod sortuniq;

use count_bytes::run as count_bytes;
use sortuniq::run as sortuniq;

fn maybe_run(name: &OsStr, args: &mut ArgsOs) -> Option<Result<u8>> {
    match name.to_str() {
        Some("true") => Some(Ok(0)),
        Some("false") => Some(Ok(1)),
        Some("count-bytes") => Some(count_bytes()),
        Some("sortuniq") => Some(sortuniq(args).map(|()| 0u8)),
        _ => None,
    }
}

fn wrapper_main(us: &OsStr) -> Result<u8> {
    eprintln!("usage: {:?} [subcommand]", us);
    eprintln!("Or, invoke with the binary name set (e.g. by symlink)");
    eprintln!();
    eprintln!("known subcommands:");
    eprintln!("  count-bytes: human representation of a byte number, and compression ratio");
    eprintln!("     sortuniq: various `| sort | uniq` constructions");
    eprintln!("  true, false: return 0/1");
    Ok(2)
}

fn run() -> Result<u8> {
    let mut args = std::env::args_os();
    let us = args.next().expect("process name should be set");
    let binary_name = Path::new(&us)
        .file_name()
        .expect("process name is non-empty");

    if let Some(result) = maybe_run(binary_name, &mut args) {
        return Ok(result?);
    }

    let first_arg = match args.next() {
        Some(x) => x,
        None => return wrapper_main(&us),
    };

    if let Some(result) = maybe_run(&first_arg, &mut args) {
        return Ok(result?);
    }

    wrapper_main(&us)
}

fn main() -> Result<()> {
    std::process::exit(i32::from(run()?))
}
