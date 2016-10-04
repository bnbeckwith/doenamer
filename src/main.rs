#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate doenamer;
use doenamer::Words;

fn main() {
    let ws = Words::new();

    let matches = App::new("Doe-namer")
        .subcommand(SubCommand::with_name("game")
                    .about("Play a rhyming game")
                    .arg(Arg::with_name("fuzz")
                         .short("z")
                         .help("Fuzz factor for correctness")
                    )
                    .arg(Arg::with_name("length")
                    .help("Length of game")))
        .subcommand(SubCommand::with_name("words")
                    .about("Known words list")
                    .arg(Arg::with_name("popular")
                         .short("p")
                         .long("popular")
                         .help("List by popularity")))
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
                    .arg(Arg::with_name("number")
                         .short("n")
                         .long("number")
                         .help("Number of results to return")
                         .takes_value(true))
                    .arg(Arg::with_name("WORD")
                         .help("Word to rhyme")
                         .index(1)
                         .required(true))).get_matches();

    match matches.subcommand() {
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
            println!("Using word: {} {}", word, ws.find_phoneme(word));

            let rhymes = ws.find_rhymes(word);
            let mut number_of_items: usize = rhymes.len();

            println!("Number of items: {}", number_of_items);

            if let Some(limit) = sub_m.value_of("number") {
                println!("limit: {}", limit);
                number_of_items = limit.parse::<usize>().unwrap_or(number_of_items);
            }

            println!("Number of items: {}", number_of_items);

            for rhyme in rhymes.iter().take(number_of_items) {
                println!("{}", rhyme);
            }},
        ("words", Some(_)) => {
            for word in ws.wordlist() {
                println!("{}", word);
            }
        },
        _ => println!("Incorrect subcommand")
    }
}
