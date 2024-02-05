use clap::Parser;
use std::{
    error::Error,
    io::{self, BufRead, BufReader, Read},
};
use std::fs::File;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(default_value = "-", help = "Input file(s)")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "lines",
        conflicts_with("bytes"),
        default_value = "10",
        help = "Number of lines"
    )]
    lines: usize,

    #[arg(short = 'c', long = "bytes", help = "Number of bytes")]
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();

    Ok(args)
}

pub fn run(args: Args) -> MyResult<()> {
    let num_files = args.files.len();
    for (idx, filename) in args.files.iter().enumerate() {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut file) => {
                if idx >= 1 {
                    println!("");
                }
                if num_files >= 2 {
                    println!("==> {} <==", filename);
                }
                match args.bytes {
                    None => { // take option n
                        let mut line = String::new();
                        for _ in 0..args.lines {
                            let bytes = file.read_line(&mut line)?;
                            if bytes == 0 { break; } // 0 means an EOF
                            print!("{}", line);
                            line.clear();
                        }
                    },
                    Some(bytes) => {
                        let mut handle = file.take(bytes as u64);
                        let mut buffer = vec![0; bytes];
                        let bytes_read = handle.read(&mut buffer)?;
                        print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    }
                }
            },
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

// clap_v4 では，コマンドライン引数のパースをclap::Parser::parse()が全部やってくれるので，parse_positive_intはなくていい
// #[allow(dead_code)]
// fn parse_positive_int(val: &str) -> MyResult<usize> {
//     let val_parsed: Result<usize, _> = val.parse();
//     match val_parsed {
//         Ok(v) if v > 0 => Ok(v),
//         _ => Err(From::from(val)),
//     }
// }

// #[test]
// fn test_parse_positive_int() {
//     let res = parse_positive_int("3");
//     assert!(res.is_ok());
//     assert_eq!(res.unwrap(), 3);
// }
