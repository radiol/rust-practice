use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "echor", author, version, about = "Rust echo")]
struct Cli {
    // Input text
    #[arg(value_name="TEXT", required=true, num_args=1..)]
    text: Vec<String>,
    // Do not print newline
    #[arg(short = 'n', long)]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();
    print!(
        "{}{}",
        cli.text.join(" "),
        if cli.omit_newline { "" } else { "\n" }
    )
}
