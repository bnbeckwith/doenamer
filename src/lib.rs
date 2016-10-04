// Structures
use std::collections::HashMap;

// Lazy Static
#[macro_use]
extern crate lazy_static;

// Regex
extern crate regex;
use regex::Regex;

// String Comparisons
extern crate strsim;
use strsim::damerau_levenshtein;

// Dictionary of phonemes
const DICT: &'static str = include_str!("data/cmudict-0.7b");

// HashMap of phoenemes
lazy_static! {
    static ref PHONEMES: HashMap<&'static str, &'static str> = {
        let mut hash = HashMap::new();
        for line in DICT.lines(){
            let mut iter = line.splitn(2,' ');
            hash.insert(iter.next().unwrap(), iter.next().unwrap());
        }
        hash
    };
}

/// The `Words` struct.
pub struct Words<'a> {
    dictionary: HashMap<&'a str, &'a str>
}

impl<'a> Words<'a> {
    pub fn new() -> Words<'a> {
        let mut hash: HashMap<&'a str, &'a str> = HashMap::new();
        for line in DICT.lines(){
            let mut iter = line.splitn(2,' ');
            hash.insert(iter.next().unwrap(), iter.next().unwrap());
        }
        Words{dictionary: hash}
    }

    pub fn wordlist(&self) -> Vec<&'a str> {
        let mut words: Vec<&'a str> = Vec::new();

        for (key, _) in self.dictionary.iter() {
            words.push(key);
        }

        words.sort();
        words
    }

    pub fn phoneme_suffix(&self,phoneme: &str) -> String {
        let s_iter = phoneme.split(" ");
        let suffix_re = Regex::new(r".*\d").unwrap();
        let sounds: Vec<&str> = s_iter.skip_while(|x| !suffix_re.is_match(x)).collect();
        sounds.join(" ")
    }

    pub fn rhymes(&self, a: &str, b: &str) -> bool {
        let suffix = self.phoneme_suffix(a);
        let re = Regex::new(&suffix).unwrap();
        re.is_match(self.find_phoneme(b))
    }

    pub fn find_rhymes(&self, word: &str) -> Vec<&'a str> {
        let phoneme: &str = self.find_phoneme(word);

        let mut rhymes: Vec<&'a str> = Vec::new();

        let suffix: String = self.phoneme_suffix(phoneme);

        println!("Suffix: {}", suffix);

        let re = Regex::new(&suffix).unwrap();
        for (key, val) in self.dictionary.iter(){
            if re.is_match(val){
                rhymes.push(key)
            }
        }

        rhymes.sort_by_key(|k| self.phoneme_distance(word, k));
        rhymes.remove(0);
        let base = self.phoneme_distance(word,rhymes.first().unwrap()) + 2;
        rhymes.retain(|x| self.phoneme_distance(x, word) <= base);

        rhymes
    }

    pub fn find_phoneme(&self, word: &str) -> &str {
        self.dictionary.get(word).expect("{} not found in dictionary")
    }

    pub fn phoneme_distance(&self, a: &str, b: &str) -> usize {
        damerau_levenshtein(self.find_phoneme(a),self.find_phoneme(b))
    }
}
