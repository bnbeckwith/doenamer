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

/// `WordsConfig` holds all configuration parameters for DOE-(wait for it)-NAMER. HAHAHAHAHA!
pub struct DoenamerConfig {
    limit: usize,
    only_common: bool,
    debug_level: u64,
    fuzz: u64
}

impl DoenamerConfig {
    pub fn new(limit: Option<usize>,
               only_common: bool,
               debug_level: u64,
               fuzz: u64
    ) -> DoenamerConfig {
        DoenamerConfig {
            only_common: only_common,
            limit: limit.unwrap_or(PHONEMES.len()),
            fuzz: fuzz,
            debug_level: debug_level
        }
    }
}

/// The `Words` struct.
pub struct Rhymely {
    config: DoenamerConfig
}

impl Rhymely {
    pub fn new(config: DoenamerConfig) -> Rhymely<> {
        Rhymely{
            config: config
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

        if self.config.only_common {
            if self.config.debug_level > 1 {
                println!("Using only common words");
            }

            let result_set: HashSet<&str> = items.into_iter().collect();
            items = COMMONS.intersection(&result_set).map(|s| *s).collect::<Vec<&str>>();
        }

        if self.config.debug_level > 1 {
            println!("Limiting to {}", self.config.limit);
        }

       items.into_iter().take(self.config.limit).collect()
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
        re.is_match(self.find_phoneme(b).unwrap())
    }

    pub fn find_rhymes<'a>(&'a self, word: &'a str) -> Result<Vec<&str>, &str> {
        let phoneme: &str = match self.find_phoneme(word) {
            Some(p) => p,
            None => return Err(word)
        };

        let mut rhymes: Vec<&str> = Vec::new();

        let suffix: String = self.phoneme_suffix(phoneme);

        if self.config.debug_level > 0 {
            println!("Suffix: {}", suffix);
        }

        let re = Regex::new(&suffix).unwrap();
        for (key, val) in PHONEMES.iter(){
            if re.is_match(val){
                rhymes.push(key)
            }
        }

        rhymes.sort_by_key(|k| self.phoneme_distance(word, k));
        // Remove yourself
        rhymes.remove(0);

        // Handle empty case here.
        if rhymes.len() < 1 {
            return Ok(rhymes)
        }

        let base = self.phoneme_distance(word, *rhymes.first().unwrap()).unwrap();
        rhymes.retain(|x| self.phoneme_distance(x, word).unwrap() <= base);
        Ok(self.compact(rhymes))
    }

    pub fn find_phoneme(&self, word: &str) -> Option<&str> {
        PHONEMES.get(word).map(|x| *x)
    }

    pub fn phoneme_distance(&self, a: &str, b: &str) -> Result<usize, String > {
        let (phoneme_a, phoneme_b) = match (self.find_phoneme(a), self.find_phoneme(b)) {
            (Some(x),Some(y)) => (x,y),
            (None, None) => return Err(format!("Can't find words: {}, {}", a, b)),
            (None, _)    => return Err(format!("Can't find word: {}", a)),
            (_, None)    => return Err(format!("Can't find word: {}", b))
        };

        Ok(damerau_levenshtein(phoneme_a,phoneme_b))
    }
}

