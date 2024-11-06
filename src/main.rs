use clap::Parser;

#[derive(Parser)]
#[command(name = "Bitcoin bip39 xor")]
#[command(version = "1.0")]
#[command(about = "Get the xor of 2 bip39 words", long_about = None)]
struct Cli {
    #[arg(required = true, help = "(string, required) english bip39 word")]
    word_a: String,
    #[arg(required = true, help = "(string, required) english bip39 word")]
    word_b: String,
}


fn main() {
    let cli = Cli::parse();
    match xor_bip39::get_xor(cli.word_a.as_str(), cli.word_b.as_str()) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e),
    }
}
