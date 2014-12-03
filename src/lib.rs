#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
use std::io::File;
use std::str::StrSlice;
use std::ascii::AsciiExt;
use std::collections::hashmap::{HashMap, Occupied, Vacant};

pub struct NwordCorrector {
    // Dictionary of words
    nwords : HashMap<String, int>
}

impl NwordCorrector {
    // Create new instance.
    pub fn new() -> NwordCorrector {
        let mut nwords : HashMap<String, int> = HashMap::new();
        
        let contents = File::open(&Path::new("big.txt"))
                            .read_to_string().unwrap();
        let re = regex!(r"[a-z]+");
        
        for cap in re.captures_iter(contents.as_slice()
                                    .to_ascii_lower().as_slice()) {
            match nwords.entry(cap.at(0).to_string()) {
                Vacant(entry) => { entry.set(1); },
                Occupied(mut entry) => { *entry.get_mut() += 1; },
            }
 
        } 
        NwordCorrector { nwords : nwords}
    }

    // Correct the given word. 
    pub fn correct(&self, word : String) -> String {
        // Word exist in dictionary, return
        if self.nwords.contains_key(&word) {
            return word
        }

        let mut candidates : HashMap<int, String> = HashMap::new();
        
        // Lets do first corrections
        let mut known_edit1 = Vec::new();
        NwordCorrector::edits(word.as_slice(), & mut known_edit1);
        for ke1 in known_edit1.iter() {
            if self.nwords.contains_key(ke1) {
                candidates.insert(self.nwords[*ke1], ke1.clone());
            }
        }

        if !candidates.is_empty() {
            return candidates[candidates.keys()
                .fold(0i, |max, x| { if *x > max { *x } else { max }})].clone()
        }

        // Lets do second corrections
        for ke1 in known_edit1.iter() {
            let mut known_edit2 = Vec::new();
            NwordCorrector::edits(ke1.as_slice(), & mut known_edit2);
            for ke2 in known_edit2.into_iter() {
                if self.nwords.contains_key(&ke2) {
                    candidates.insert(self.nwords[ke2], ke2);
                }
            }
        }
        if !candidates.is_empty() {
            return candidates[candidates.keys()
                .fold(0i, |max, x| { if *x > max { *x } else { max }})].clone()
        }

        // No corrections found
        word
    }


    fn edits (word : &str, edits :& mut Vec<String>) {
        // Delete
        for i in range(0, word.len()) {
            edits.push(format!("{}{}", word.slice_to(i), word.slice_from(i+1)));
        }

        // Transpose
        for i in range(0, word.len() - 1) {
            edits.push(format!("{}{}{}{}", word.slice_to(i), 
                               word.slice(i+1, i+2), 
                               word.slice(i, i+1), word.slice_from(i+2)));
        }

        // Replaces
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        for i in range(0, word.len()) {
            for c in alphabet.chars() {
                edits.push(format!("{}{}{}", word.slice_to(i), c, word.slice_from(i+1)));
            }
        }

        // Insert
        for i in range(0, word.len()) {
            for c in alphabet.chars() {
                edits.push(format!("{}{}{}", word.slice_to(i), c, word.slice_from(i)));
            }
        }
    }
}
