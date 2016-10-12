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

pub struct ScoredWord<'a> {
    word: &'a str,
    score: usize
}

/// `WordsConfig` holds all configuration parameters for DOE-(wait for it)-NAMER. HAHAHAHAHA!
pub struct DoenamerConfig {
    limit: usize,
    only_common: bool,
    debug_level: u64,
    fuzz: u64,
    homophones: bool,
}

impl DoenamerConfig {
    pub fn new(limit: Option<usize>,
               only_common: bool,
               debug_level: u64,
               fuzz: u64,
               homophones: bool
    ) -> DoenamerConfig {
        DoenamerConfig {
            only_common: only_common,
            limit: limit.unwrap_or(PHONEMES.len()),
            fuzz: fuzz,
            debug_level: debug_level,
            homophones: homophones
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

        let mut rhymes: Vec<ScoredWord> = Vec::new();

        let suffix: String = self.phoneme_suffix(phoneme);

        if self.config.debug_level > 0 {
            println!("Suffix: {}", suffix);
        }

        let re = Regex::new(&suffix).unwrap();
        for (key, val) in PHONEMES.iter() {
            if *key == word {
                continue;
            }
            if re.is_match(val) {
                let score = self.phoneme_distance(word, key).unwrap();

                if (score > 0) || self.config.homophones {
                    rhymes.push(
                        ScoredWord {
                            word: key,
                            score: score
                        })
                }
            }
        }

        // Sort by scoring
        rhymes.sort_by_key(|k| k.score);

        let mut num_changes = self.config.fuzz + 2;
        if self.config.homophones {
            num_changes += 1
        }
        let mut current_score = 0;
        let mut results: Vec<&str> = Vec::new();

        for rhyme in &rhymes {
            if rhyme.score > current_score {
                num_changes -= 1;
                current_score = rhyme.score;
            }
            if num_changes == 0 { break; }

            results.push(rhyme.word)
        }

        Ok(self.compact(results))
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

