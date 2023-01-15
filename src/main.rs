use rayon::prelude::*;
use lindera::tokenizer::{Token, Tokenizer};
use std::{
    collections::HashMap,
    env,
    fs,
    io::Write,
};

fn tokenize<'a>(tokenizer: &'a mut Tokenizer, target_text: &'a str) -> Vec<Token<'a>> {
    let tokens = tokenizer
        .tokenize(target_text)
        .expect("failed to tokenize");
    return tokens;
}

#[test]
fn test_tokenize() {
    let mut tokenizer = Tokenizer::new().unwrap();
    let target_text = "今日は晴れだ。".to_string();
    let tokens = tokenize(&mut tokenizer, &target_text);
    for token in &tokens {
        println!("{:?}", token.detail);
    }
    assert_eq!(tokens.len(), 5);
}

#[test]
fn test_word_count() {
    let mut tokenizer = Tokenizer::new().unwrap();
    let mut word_counter = HashMap::new();
    let target_text = "今日は晴れだ。今日は雨が降らない。明日も晴れだ。".to_string();
    let tokens = tokenize(&mut tokenizer, &target_text);
    for token in &tokens {
        let word = token.text;
        let entry = word.to_string();
        word_counter.entry(entry).and_modify(|counter| *counter += 1).or_insert(1);
    }
    assert_eq!(word_counter.get("今日").unwrap(), &2);
    assert_eq!(word_counter.len(), 11)
}

fn write_result(word_counter: &HashMap<String, i32>, result_path: &String) -> std::io::Result<()> {
    let file = fs::File::create(result_path).unwrap();
    let words_counts_text = serde_json::to_string(&word_counter).unwrap();
    write!(&file, "{}", words_counts_text)?;
    Ok(())
}

fn count_word(tokenizer: &mut Tokenizer, text: &str) -> HashMap<String, i32> {
    let tokens = tokenize(tokenizer, text);
    let mut word_counter = HashMap::new();
    for token in &tokens {
        let word = token.text;
        let entry = word.to_string();
        word_counter.entry(entry).and_modify(|counter| *counter += 1).or_insert(1);
    }
    return word_counter;
}

fn main() {
    let mut word_counter = HashMap::new();
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];
    let result_path = &args[2];

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let count = contents.par_lines().map_init(
        || Tokenizer::new().unwrap(),
        | tokenizer, line | count_word(tokenizer, line));
    let map_list: Vec<HashMap<String, i32>> = count.collect();
    for map in map_list {
        for (key, value) in map {
            word_counter.entry(key).and_modify(|counter| *counter += value).or_insert(value);
        }
    }
    println!("{:?}", word_counter.len());
    write_result(&word_counter, &result_path).unwrap();
}
