// modules
pub mod game;

// Random utilities
extern crate rand;

// Structures
use std::collections::HashMap;
use std::collections::HashSet;

// Lazy Static
#[macro_use]
extern crate lazy_static;

// Regex
extern crate regex;
use regex::Regex;

// String Comparisons
extern crate strsim;
use strsim::damerau_levenshtein;

// Dictionary of most common words
const MOSTCOMMON: &'static str = include_str!("data/mostcommon");

// List of most common words
lazy_static! {
    pub static ref COMMONS: HashSet<&'static str> = {
        let mut words = HashSet::new();
        for line in MOSTCOMMON.lines() {
            words.insert(line);
        }
        words
    };
}

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
pub struct Words<> {
    limit: usize,
    only_common: bool,
    debug_level: u64
}

impl<> Words<> {
    pub fn new(only_common: bool, limit: Option<usize>, debug_level: u64) -> Words<> {
        Words{
            only_common: only_common,
            limit: limit.unwrap_or(PHONEMES.len()),
            debug_level: debug_level
        }
    }

    pub fn wordlist(&self) -> Vec<&str> {
        let mut words: Vec<&str> = Vec::new();

        for (key, _) in PHONEMES.iter() {
            words.push(key);
        }

        words.sort();
        words
    }

    fn compact<'a>(&'a self, mut items: Vec<&'a str>) -> Vec<&str> {

        if self.only_common {
            if self.debug_level > 1 {
                println!("Using only common words");
            }

            let result_set: HashSet<&str> = items.into_iter().collect();
            items = COMMONS.intersection(&result_set).map(|s| s.to_owned()).collect::<Vec<&str>>();
        }

        if self.debug_level > 1 {
            println!("Limiting to {}", self.limit);
        }

       items.into_iter().take(self.limit).collect()
    }

    pub fn common(&self) -> Vec<&str> {
        let mut words: Vec<&str> = Vec::new();

        for word in COMMONS.iter() {
            words.push(word);
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

    pub fn find_rhymes(&self, word: &str) -> Vec<&str> {
        let phoneme: &str = self.find_phoneme(word);

        let mut rhymes: Vec<&str> = Vec::new();

        let suffix: String = self.phoneme_suffix(phoneme);

        if self.debug_level > 0 {
            println!("Suffix: {}", suffix);
        }

        let re = Regex::new(&suffix).unwrap();
        for (key, val) in PHONEMES.iter(){
            if re.is_match(val){
                rhymes.push(key)
            }
        }

        rhymes.sort_by_key(|k| self.phoneme_distance(word, k));
        rhymes.remove(0);
        let base = self.phoneme_distance(word,rhymes.first().unwrap()) + 2;
        rhymes.retain(|x| self.phoneme_distance(x, word) <= base);

        self.compact(rhymes)
    }

    pub fn find_phoneme(&self, word: &str) -> &str {
        PHONEMES.get(word).expect("{} not found in dictionary")
    }

    pub fn phoneme_distance(&self, a: &str, b: &str) -> usize {
        damerau_levenshtein(self.find_phoneme(a),self.find_phoneme(b))
    }

}

