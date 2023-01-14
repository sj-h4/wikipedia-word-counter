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
    let target_text = "今日は晴れだ。".to_string();
    let tokens = tokenize(&target_text);
    for token in &tokens {
        println!("{:?}", token.detail);
    }
    assert_eq!(tokens.len(), 5);
}

#[test]
fn test_word_count() {
    let mut word_counter = HashMap::new();
    let target_text = "今日は晴れだ。今日は雨が降らない。明日も晴れだ。".to_string();
    let tokens = tokenize(&target_text);
    for token in &tokens {
        let word = token.text;
        let entry = word.to_string();
        word_counter.entry(entry).and_modify(|counter| *counter += 1).or_insert(1);
    }
    assert_eq!(word_counter.get("今日").unwrap(), &2);
    assert_eq!(word_counter.len(), 11)
}

fn main() {
    let mut word_counter = HashMap::new();
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for line in lines {
        let line = line.unwrap();
        let tokens = tokenize(&line);
        for token in &tokens {
            let word = token.text;
            let entry = word.to_string();
            word_counter.entry(entry).and_modify(|counter| *counter += 1).or_insert(1);
        }
    }
    println!("{:?}", word_counter.len());
}
