use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(value_name("FILES"), default_value("-"))]
    files: Vec<String>,
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> MyResult<()> {
    for filename in cli.files {
        match open(&filename) {
            Err(e) => eprintln!("catr: {filename}: {e}"),
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    match (cli.number_lines, cli.number_nonblank_lines) {
                        (true, _) => println!("{:6}\t{line}", line_num + 1),
                        (_, true) => {
                            if !line.is_empty() {
                                prev_num += 1;
                                println!("{:6}\t{line}", prev_num);
                            } else {
                                println!();
                            }
                        }
                        (_, _) => println!("{line}"),
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
