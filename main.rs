use std::collections::HashMap;
use serde::Serialize;
use std::env;

#[derive(Debug, Serialize)]
struct Xor {
    word_a: String,
    word_b: String,
    xor_word: String,
}

fn create_word_to_index_map(words: &[&str]) -> HashMap<String, u16> {
    words
        .iter()
        .enumerate()
        .map(|(i, &word)| (word.to_string(), i as u16))
        .collect()
}

fn word_to_bits(word_to_index: &HashMap<String, u16>, word: &str) -> u16 {
    match word_to_index.get(word) {
        Some(&index) => index,
        None => {
            eprintln!("Word '{}' not found in BIP39 list", word);
            std::process::exit(1);
        }
    }
}

fn xor_bits(bits1: u16, bits2: u16) -> u16 {
    bits1 ^ bits2
}

fn get_bip39_index_by_word(words: &[&str], word: &str) -> u16 {
    let word_to_index = create_word_to_index_map(&words);
    let bits = word_to_bits(&word_to_index, &word);
    bits
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <word1> <word2>", args[0]);
        std::process::exit(1);
    }

    let word_a = args[1].to_string();
    let word_b = args[2].to_string();

    let bip39_words = include_str!("bip39-english.txt")
        .lines()
        .collect::<Vec<_>>();

    let bits1 = get_bip39_index_by_word(&bip39_words, &word_a.as_str());
    let bits2 = get_bip39_index_by_word(&bip39_words, &word_b.as_str());

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
        word_a,
        word_b,
        xor_word: xor_word.to_string(),
    });

    println!("Xor: {}", serde_json::to_string_pretty(&xor_result).unwrap());
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
