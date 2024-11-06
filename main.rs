use clap::Parser;
use std::io::Error;
use xor::{get_bip39_index_by_word, xor_bits, Xor};
mod xor;

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

fn get_xor(word_a: &str, word_b: &str) -> Result<String, Error> {
    let bip39_words = include_str!("bip39-english.txt")
        .lines()
        .collect::<Vec<_>>();

    let bits1 = get_bip39_index_by_word(&bip39_words, &word_a);
    let bits2 = get_bip39_index_by_word(&bip39_words, &word_b);

    let xor_result = xor_bits(bits1, bits2);

    let xor_word = match bip39_words.get(xor_result as usize) {
        Some(&word) => word,
        None => {
            eprintln!("XOR not possible in Bip 39 range.");
            std::process::exit(1);
        }
    };

    let mut xor_result = vec![];
    xor_result.push(Xor {
        word_a: word_a.to_string(),
        word_b: word_b.to_string(),
        xor_word: xor_word.to_string(),
    });
    Ok(serde_json::to_string_pretty(&xor_result)?)
}

fn main() {
    let cli = Cli::parse();
    match get_xor(cli.word_a.as_str(), cli.word_b.as_str()) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::get_bip39_index_by_word;

    #[test]
    fn test_get_bip39_index_by_word() {
        let bip39_words = include_str!("bip39-english.txt")
            .lines()
            .collect::<Vec<_>>();

        let all_word = "all";
        let bits = get_bip39_index_by_word(&bip39_words, &all_word);
        assert_eq!(bits, 0b00000110011_u16);
    }
}
