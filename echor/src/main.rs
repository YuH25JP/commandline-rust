use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true, help = "some help")]
    text: Vec<String>,

    #[arg(short = 'n', help = "Do not print newline")]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    // println!("{:?}", args);

    let text_vec = args.text;
    let omit_newline = args.omit_newline;
    let mut ending = "\n";
    if omit_newline {
        ending = "";
    }

    print!("{}{}", text_vec.join(" "), ending);
}
