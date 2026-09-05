mod app;
mod audio;
mod tui;
mod scanner;

use std::{env, path::PathBuf, process};

use app::App;
use getopts::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let prog = &args[0];

    let mut opts = Options::new();
    opts.optopt("p", "path", "Path to music directory", "DIR");
    opts.optflag("h", "help", "print usage and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{prog}: Error: {f}");
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(prog, &opts);
        process::exit(0);
    }

    let music_path = match matches.opt_str("p") {
        Some(x) => PathBuf::from(x),
        None => {
            eprintln!("{prog}: Error: Music path is required.\n");
            print_usage(prog, &opts);
            process::exit(1);
        }
    };

    println!("{}", music_path.display());
    let app = App::new()?;

    app.run()?;

    Ok(())
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {program} [options]");
    print!("{}", opts.usage(&brief));
}
