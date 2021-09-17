use clap::Arg;

use std::{
    collections::HashSet,
    error::Error,
    ffi::OsString,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

struct Config {
    verbose_interval: Option<usize>,
    infile_path: OsString,
}

fn main() {
    match main_err() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("fatal error: {}", e);

            std::process::exit(1)
        }
    }
}

fn main_err() -> Result<(), Box<dyn Error>> {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .value_name("INTERVAL")
                .default_value("10000")
                .help("Print out a diagnostic to stderr every INTERVAL lines"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .value_name("FILE")
                .default_value("-")
                .help("File to read from, or - for stdin"),
        )
        .get_matches_safe()?;

    let verbose_interval: Option<usize> = if matches.occurrences_of("verbose") > 0 {
        let verbose_val = matches
            .value_of("verbose")
            .expect("default specified for verbose");

        Some(verbose_val.parse().map_err(|e| {
            format!(
                "Could not parse verbose flag: {} while parsing {:?}",
                e, verbose_val
            )
        })?)
    } else {
        None
    };

    process(&Config {
        verbose_interval,
        infile_path: matches.value_of_os("file").unwrap().to_owned(),
    })
}

fn process(conf: &Config) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let infile: Box<dyn BufRead> = if &conf.infile_path == "-" {
        Box::new(stdin.lock())
    } else {
        Box::new(BufReader::new(File::open(&conf.infile_path)?))
    };

    let stdout_ref = io::stdout();
    let mut stdout = stdout_ref.lock();

    let mut seen_lines = HashSet::<String>::new();
    let mut line_count: usize = 0;

    for line in infile.lines() {
        line_count += 1;

        let line = line?;

        if !seen_lines.contains(&line) {
            stdout.write_all(line.as_bytes())?;
            stdout.write_all(LINE_ENDING.as_bytes())?;

            seen_lines.insert(line);
        }

        // If requested, print out statistics to stderr
        if let Some(vi) = conf.verbose_interval {
            if line_count % vi == 0 {
                eprintln!(
                    "lines: {}, uniques: {}, {:.5}% unique",
                    line_count,
                    seen_lines.len(),
                    seen_lines.len() as f32 / line_count as f32 * 100.
                )
            }
        }
    }

    Ok(())
}
