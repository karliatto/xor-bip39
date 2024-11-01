use std::env;
use xor::{Xor, get_bip39_index_by_word, xor_bits};
mod xor;

fn get_xor(word_a: &str, word_b: &str) -> Vec<Xor> {

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
    xor_result
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <word1> <word2>", args[0]);
        std::process::exit(1);
    }

    let word_a = &args[1];
    let word_b = &args[2];

    let xor_result = get_xor(word_a.as_str(), word_b.as_str());

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
