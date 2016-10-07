use rand::{thread_rng, sample};
use COMMONS;

pub struct Game<'a> {
    guesses: Vec<&'a str>
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        Game { guesses: Vec::new() }
    }

    // 1. Give a random word
    // 2. User gives rhyme
    // 3. Check to see that it isn't a duplication
    //   -- Add to guesses
    //   -- if duplication, user loses
    // 4. Search for rhymes not already used
    // 5. If none, user wins.
    // 6. Respond with rhyme of users word.
    //   -- Add to guesses
    // 7. Loop back to 2

    fn guess(&self, word: &'a str) -> Result<&'a str, &'static str> {
        Ok(word)
    }

    fn set_start_word(&mut self) {
        let mut rng = thread_rng();
        let sample = sample(&mut rng, COMMONS.iter(), 1);
        self.guesses.push(sample[0]);
    }

    pub fn run() {
        let mut game = Game::new();
        println!("Running the game");
        game.set_start_word();
        while let Ok(word) = game.guess(game.guesses[0]){
            println!("{}", word)
        }
    }
}
