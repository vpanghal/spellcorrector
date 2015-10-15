#![feature(plugin)]
#![plugin(regex_macros)]

extern crate regex;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;
use std::string::String;
use regex::Regex;

pub struct SpellCorrector {
    // Dictionary of words
    nwords: HashMap<String, isize>,
}

impl SpellCorrector {
    // Create new instance.
    pub fn new() -> SpellCorrector {
        let mut nwords: HashMap<String, isize> = HashMap::new();

        let path = Path::new("big.txt");
        let file = match File::open("big.txt") {
            Err(why) => panic!("failed to open {}: {}", path.display(), why),
            Ok(f) => f,
        };

        let re = Regex::new(r"\w+").unwrap();
        for line in BufReader::new(file).lines() {
            for cap in re.captures_iter(&line.unwrap()) {
                let counter = nwords.entry(String::from(cap.at(0).unwrap())).or_insert(0);
                *counter += 1;
            }
        }

        SpellCorrector { nwords: nwords }
    }

    // Correct the given word.
    pub fn correct(&self, word: String) -> String {
        // Word exist in dictionary, return
        if self.nwords.contains_key(&word) {
            return word;
        }

        // Find corrections with distance of 1
        let mut known_edit1 = Vec::new();
        SpellCorrector::edits(&word, &mut known_edit1);
        let mut word_count = 0;
        let mut correction: Option<&String> = None;
        for ke1 in &known_edit1 {
            if self.nwords.contains_key(ke1) {
                let ke1_count = self.nwords[ke1];
                if ke1_count > word_count {
                    word_count = ke1_count;
                    correction = Some(&ke1);
                }
            }
        }

        match correction {
            Some(word) => {
                return (*word).clone();
            }
            None => {}
        }

        // Find corrections with distance of 2
        let mut correction2: Option<String> = None;
        for ke1 in known_edit1.iter() {
            let mut known_edit2 = Vec::new();
            SpellCorrector::edits(&ke1, &mut known_edit2);
            for ke2 in &known_edit2 {
                if self.nwords.contains_key(ke2) {
                    let ke2_count = self.nwords[ke2];
                    if ke2_count > word_count {
                        word_count = ke2_count;
                        // Drop existing string
                        drop(correction2);
                        correction2 = Some(String::from(ke2.clone()));
                    }
                }
            }
        }

        match correction2 {
            Some(word) => {
                return word;
            }
            None => {}
        }

        // No corrections found
        word
    }

    fn edits(word: &String, edits: &mut Vec<String>) {
        // Delete
        let len = word.len();
        for i in 0..len {
            edits.push(format!("{}{}", &word[0..i], &word[i + 1..]));
        }

        // Transpose
        for i in 0..(len - 1) {
            edits.push(format!("{}{}{}{}",
                               &word[0..i],
                               &word[i + 1..i + 2],
                               &word[i..i + 1],
                               &word[i + 2..]));
        }

        // Replaces
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        for i in 0..len {
            for c in alphabet.chars() {
                edits.push(format!("{}{}{}", &word[0..i], c, &word[i + 1..]));
            }
        }

        // Insert
        for i in 0..len {
            for c in alphabet.chars() {
                edits.push(format!("{}{}{}", &word[0..i], c, &word[i..]));
            }
        }
    }
}
