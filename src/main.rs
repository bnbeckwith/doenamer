#[macro_use]
extern crate clap;
use clap::App;

// Structures
use std::collections::HashMap;

// Regex
extern crate regex;
use regex::Regex;

// String Comparisons
extern crate strsim;
use strsim::damerau_levenshtein;

const DICT: &'static str = include_str!("data/cmudict-0.7b");

struct Words<'a> {
    dictionary: HashMap<&'a str, &'a str>
}

impl<'a> Words<'a> {
    fn new() -> Words<'a> {
        let mut hash: HashMap<&'a str, &'a str> = HashMap::new();
        for line in DICT.lines(){
            let mut iter = line.splitn(2,' ');
            hash.insert(iter.next().unwrap(), iter.next().unwrap());
        }
        Words{dictionary: hash}
    }

    fn phoneme_suffix(&self,phoneme: &str) -> String {
        let s_iter = phoneme.split(" ");
        let suffix_re = Regex::new(r".*\d").unwrap();
        let sounds: Vec<&str> = s_iter.skip_while(|x| !suffix_re.is_match(x)).collect();
        sounds.join(" ")
    }

    fn rhymes(&self, a: &str, b: &str) -> bool {
        let suffix = self.phoneme_suffix(a);
        let re = Regex::new(&suffix).unwrap();
        re.is_match(self.find_phoneme(b))
    }

    fn find_rhymes(&self, word: &str) -> Vec<&'a str> {
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

        rhymes.sort_by(|a, b| damerau_levenshtein(a, phoneme).cmp(&damerau_levenshtein(b, phoneme)));
        rhymes
    }

    fn find_phoneme(&self, word: &str) -> &str {
        let phoneme: &str;
        match self.dictionary.get(word) {
            Some(s) => phoneme = s,
            None => panic!("{} not found in dictionary.")
        }
        phoneme
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let ws = Words::new();

    let word: &str = &matches.value_of("WORD").unwrap().to_uppercase();
    println!("Using word: {} {}", word, ws.find_phoneme(word));

    for rhyme in ws.find_rhymes(word).iter() {
        println!("{}", rhyme)
    }
}
