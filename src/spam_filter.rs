use crate::common::BoW;
use std::collections::HashSet;
use std::env;
use std::fs;
mod common;
use crate::common::tokenize;

const HAM_PROBABILITY: f64 = 0.5;
const SPAM_PROBABILITY: f64 = 1.0 - HAM_PROBABILITY;

fn usage(program_name: &str) {
    println!("Usage: {program_name} <file>");
}

fn get_total_word_count(bow: &BoW) -> u32 {
    bow.iter().fold(0, |acc, (_, v)| acc + v)
}

fn probability_of_word_in_bow(word: &str, total_words_in_bow: u32, vocab_size: usize, bow: &BoW) -> f64 {
    let count = bow.get(word).copied().unwrap_or(0);
    (count as f64 + 1.0) / (total_words_in_bow as f64 + vocab_size as f64)
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("ERROR: too few arguements provided");
        usage(&args[0]);
        std::process::exit(69);
    }
    let file_path = &args[1];
    let spam_bow: BoW = serde_json::from_str(&fs::read_to_string("spam.json").unwrap()).unwrap();
    let ham_bow: BoW = serde_json::from_str(&fs::read_to_string("ham.json").unwrap()).unwrap();
    let vocab_size = spam_bow
        .keys()
        .chain(ham_bow.keys())
        .collect::<HashSet<_>>()
        .len();
    let total_spam_words = get_total_word_count(&spam_bow);
    let total_ham_words = get_total_word_count(&ham_bow);
    let file = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("ERROR: could not open {file_path}: {err}");
            std::process::exit(69);
        }
    };
    let spam_ham_log = (SPAM_PROBABILITY / HAM_PROBABILITY).ln();
    let sum_of_ratio_spam_over_ham = file
        .split_whitespace()
        .map(tokenize)
        .filter(|w| !w.is_empty())
        .map(|w| {
            let s = probability_of_word_in_bow(&w, total_spam_words, vocab_size, &spam_bow);
            let h = probability_of_word_in_bow(&w, total_ham_words, vocab_size, &ham_bow);
            (s / h).ln()
        })
        .sum::<f64>();
    let spam_likelihood = sum_of_ratio_spam_over_ham + spam_ham_log;
    if spam_likelihood > 0.0 {
        println!("INFO: Document {file_path} is spam");
    } else {
        println!("INFO: Document {file_path} is not spam");
    }
}
