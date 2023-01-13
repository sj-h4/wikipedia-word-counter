use lindera::tokenizer::{Token, Tokenizer};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn tokenize(target_text: &String) -> Vec<Token> {
    let tokenizer = Tokenizer::new().unwrap();
    let tokens = tokenizer
        .tokenize(&target_text)
        .expect("failed to tokenize");
    return tokens;
}

#[test]
fn test_tokenize() {
    let target_text = "ナイフで木を削った。".to_string();
    let tokens = tokenize(&target_text);
    for token in &tokens {
        println!("{:?}", token.detail);
    }
    assert_eq!(tokens.len(), 4);
}

fn main() {
    let mut word_counter = HashMap::new();
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let tokens = tokenize(&line);
        for token in &tokens {
            let word = token.text;
            let count = word_counter.entry(word).or_insert(0);
            *count += 1;
        }
    }
}
