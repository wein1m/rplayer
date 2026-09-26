mod app;
mod audio;
mod tui;
mod scanner;
mod mpris;

use std::{env, fs, path::PathBuf, process};

use app::App;
use getopts::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let prog = &args[0];

    let mut opts = Options::new();
    opts.optopt("p", "path", "Path to music directory", "DIR");
    opts.optflag("s", "shuffle", "shuffle songs");
    opts.optflag("h", "help", "print usage and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{prog}: Error: {f}");
            process::exit(1);
        }
    };

    if args.len() == 1 {
        print_usage(prog, &opts);
        process::exit(0);
    }

    if matches.opt_present("h") {
        print_usage(prog, &opts);
        process::exit(0);
    }

    let shuffle = matches.opt_present("s");

    let music_path = match matches.opt_str("p") {
        Some(x) => {
            get_path(x.as_str())
        },
        None => {
            if let Some(path) = matches.free.first() {
                get_path(path)
            } else {
                eprintln!("{prog}: Error: Music path is required.\n");
                print_usage(prog, &opts);
                process::exit(1);
            }
        }
    };

    let mut app = App::new(music_path, shuffle)?;

    app.run()?;

    Ok(())
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {program} [options]");
    print!("{}", opts.usage(&brief));
}

fn get_path(path: &str) -> PathBuf {
    let full_path = match fs::canonicalize(&path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to get full path. {e}");
            process::exit(1);
        }
    };

    PathBuf::from(full_path)
}
