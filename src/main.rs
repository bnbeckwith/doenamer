#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate doenamer;
use doenamer::Words;

fn main() {

    let matches = App::new("Doe-namer")
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
        .subcommand(SubCommand::with_name("game")
                    .about("Play a rhyming game")
                    .arg(Arg::with_name("fuzz")
                         .short("z")
                         .help("Fuzz factor for correctness"))
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
                    .arg(Arg::with_name("fuzz")
                         .short("z")
                         .long("fuzz"))
                    .arg(Arg::with_name("WORD")
                         .help("Word to rhyme")
                         .index(1)
                         .required(true))).get_matches();

    let mut limit = None;
    if let Some(number) = matches.value_of("number") {
        limit = number.parse::<usize>().ok();
    }

    let ws = Words::new(matches.is_present("common-only"),
                        limit,
                        matches.occurrences_of("D"));

    match matches.subcommand() {
        ("game", _) =>
        {
            doenamer::game::Game::run();
        }
        ("common", _) =>
        {
            for word in ws.common().iter() {
                println!("{}", word);
            }
        },
        ("distance", Some(sub_m)) =>
        {
            println!("Distance of {} to {}: {}",
                     sub_m.value_of("WORD1").unwrap().to_uppercase(),
                     sub_m.value_of("WORD2").unwrap().to_uppercase(),
                     ws.phoneme_distance(
                         &sub_m.value_of("WORD1").unwrap().to_uppercase(),
                         &sub_m.value_of("WORD2").unwrap().to_uppercase()))},
        ("rhyme", Some(sub_m)) => {
            let word: &str = &sub_m.value_of("WORD").unwrap().to_uppercase();
            //println!("Using word: {} {}", word, ws.find_phoneme(word));

            let rhymes = ws.find_rhymes(word);

            for rhyme in rhymes.iter() {
                println!("{}", rhyme);
            }},
        ("words", Some(_)) => {
            for word in ws.wordlist() {
                println!("{}", word);
            }
        },
        _ => println!("Unimplemented subcommand")
    }
}
