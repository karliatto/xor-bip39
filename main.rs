use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <word1> <word2>", args[0]);
        std::process::exit(1);
    }

    let bip39_words = include_str!("bip39-english.txt").lines().collect::<Vec<_>>();
    let word_to_index = create_word_to_index_map(&bip39_words);

    let bits1 = word_to_bits(&word_to_index, &args[1]);
    println!("Word 1 bits: {:011b}", bits1);
    let bits2 = word_to_bits(&word_to_index, &args[2]);
    println!("Word 2 bits: {:011b}", bits2);

    let xor_result = xor_bits(bits1, bits2);
    println!("XOR result: {:011b}", xor_result);


    let xor_word = match bip39_words.get(xor_result as usize) {
        Some(&word) => word,
        None => {
            eprintln!("XOR result out of BIP39 range");
            std::process::exit(1);
        }
    };

    println!("XOR Result Word: {}", xor_word);
}



fn create_word_to_index_map(words: &[&str]) -> HashMap<String, u16> {
    words.iter().enumerate().map(|(i, &word)| (word.to_string(), i as u16)).collect()
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



#[cfg(test)]
mod tests {
    #[test]
    fn seed_xor_works() {

        let a_str = "romance wink lottery autumn shop bring dawn tongue range crater truth ability miss spice fitness easy legal release recall obey exchange recycle dragon room";
        let b_str = "lion misery divide hurry latin fluid camp advance illegal lab pyramid unaware eager fringe sick camera series noodle toy crowd jeans select depth lounge";
        let results_str = "defy island room gas rookie easily blame travel school excess egg unable since milk mother grace rocket case fence photo decorate idle junior cross";


        let a_vec: Vec<&str> = a_str.split_whitespace().collect();
        let b_vec: Vec<&str> = b_str.split_whitespace().collect();
        let results_vec: Vec<&str> = results_str.split_whitespace().collect();

        // Example of accessing elements
        println!("First word of a_vec: {}", a_vec[0]);
        println!("First word of b_vec: {}", b_vec[0]);
        println!("First word of results_vec: {}", results_vec[0]);

        for (index, word) in a_vec.iter().enumerate()  {
            println!("{}", word);
            println!("{}", b_vec[index]);
            // let xor_result =
        }

        // let a = Mnemonic::from_str(a_str).unwrap();
        // let b = Mnemonic::from_str(b_str).unwrap();
        // let c = Mnemonic::from_str(c_str).unwrap();
        // let result = Mnemonic::from_str(result_str).unwrap();

        // assert_eq!(result, a.clone() ^ b.clone() ^ c.clone());
        // assert_eq!(result, b ^ c ^ a); // Commutative


    }
}
