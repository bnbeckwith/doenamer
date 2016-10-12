#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate doenamer;
use doenamer::{DoenamerConfig,Rhymely};

fn main() {

    let matches = App::new("Doe-namer")
        .arg(Arg::with_name("single-line")
             .short("1")
             .help("Single-line output"))
        .arg(Arg::with_name("D")
             .short("D")
             .multiple(true)
             .help("Set the level of debug information"))
        .arg(Arg::with_name("number")
             .short("n")
             .long("number")
             .help("Number of results to return")
             .takes_value(true))
        .arg(Arg::with_name("common-only")
             .short("c")
             .long("common-only")
             .help("Restrict results to the most common words"))
        .arg(Arg::with_name("fuzz")
             .short("z")
             .multiple(true)
             .help("Fuzz factor for correctness"))
        .subcommand(SubCommand::with_name("game")
                    .about("Play a rhyming game")
                    .arg(Arg::with_name("length")
                    .help("Length of game")))
        .subcommand(SubCommand::with_name("words")
                    .about("Known words list"))
        .subcommand(SubCommand::with_name("common")
                    .about("List of most common words"))
        .subcommand(SubCommand::with_name("distance")
                    .about("Edit distance between words")
                    .version("0.1")
                    .arg(Arg::with_name("WORD1")
                         .help("First word")
                         .index(1)
                         .required(true))
                    .arg(Arg::with_name("WORD2")
                         .help("Second word")
                         .index(2)
                         .required(true)))
        .subcommand(SubCommand::with_name("rhyme")
                    .about("Finds rhymes")
                    .version("0.1")
                    .arg(Arg::with_name("WORD")
                         .help("Word to rhyme")
                         .index(1)
                         .required(true))).get_matches();

    let mut limit = None;
    if let Some(number) = matches.value_of("number") {
        limit = number.parse::<usize>().ok();
    }

    let single_line = matches.is_present("single-line");

    let rhmly = Rhymely::new(
        DoenamerConfig::new(
            limit,
            matches.is_present("common-only"),
            matches.occurrences_of("z"),
            matches.occurrences_of("D")
        ));

    match matches.subcommand() {
        ("game", _) =>
        {
            doenamer::game::Game::run();
        }
        ("common", _) =>
        {
            for word in rhmly.common().iter() {
                println!("{}", word);
            }
        },
        ("distance", Some(sub_m)) =>
        {
            let (word1, word2) =
                (sub_m.value_of("WORD1").unwrap().to_uppercase(),
                 sub_m.value_of("WORD2").unwrap().to_uppercase());
            match rhmly.phoneme_distance(word1.as_str(),word2.as_str()) {
                Ok(distance) => println!("Distance of {} to {}: {}",
                                         word1,
                                         word2,
                                         distance),
                Err(error)   => println!("Error: {}", error)
            }
        },
        ("rhyme", Some(sub_m)) => {
            let word: &str = &sub_m.value_of("WORD").unwrap().to_uppercase();
            //println!("Using word: {} {}", word, ws.find_phoneme(word));

            match rhmly.find_rhymes(word){
                Ok(rhymes) => {
                    if single_line {
                        println!("{}", rhymes.join(" "));
                    }else{
                        for rhyme in rhymes.iter() {
                            println!("{}", rhyme);
                        }
                    }
                },
                Err(error) => println!("Error: Word not found, {}", error)
            };
        },
        ("words", Some(_)) => {
            for word in rhmly.wordlist() {
                println!("{}", word);
            }
        },
        _ => println!("Unimplemented subcommand")
    }
}
