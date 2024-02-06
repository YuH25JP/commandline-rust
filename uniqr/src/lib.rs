use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(default_value = "-", help = "Input file(s)")]
    in_file: String,

    #[arg(help = "Output file")]
    out_file: Option<String>,

    #[arg(
        short = 'c',
        long = "count",
        help = "prefix lines by the number of occurrences"
    )]
    count: bool,
}

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();
    Ok(args)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn format_string(flag: &bool, pair: &(i32, String)) -> String {
    match flag {
        true => format!("{:>4} {}", pair.0, pair.1),
        false => format!("{}", pair.1),
    }
}

pub fn run(args: Args) -> MyResult<()> {
    let mut file = open(&args.in_file).map_err(|e| format!("{}: {}", args.in_file, e))?;
    let mut out_file: Box<dyn Write> = match &args.out_file {
        Some(out_file_name) => Box::new(File::create(out_file_name)?),
        _ => Box::new(io::stdout()),
    };
    let mut line = String::new();
    let mut pair = (0, "".to_string());
    let mut first = true;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 { break; }

        if first {
            pair.0 = 1;
            pair.1 = line.clone();
            first = false;
        } else {
            if pair.1.trim_end() == line.trim_end() {
                pair.0 += 1;
            } else {
                write!(out_file, "{}", format_string(&args.count, &pair))?;
                
                pair.0 = 1;
                pair.1 = line.clone();
            }
        }

        line.clear();
    }
    if pair.0 != 0 { // when the input file is empty, this line is skipped
        write!(out_file, "{}", format_string(&args.count, &pair))?;
    }
    Ok(())
}
