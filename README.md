# wordle-inator

Lightweight simulation of the game [Wordle](https://www.powerlanguage.co.uk/wordle/) for running monte carlo tests with various strategies.

Initial strategy is based on [this](https://www.royvanrijn.com/blog/2022/01/wordle-bot/) blog post by [@royvanrijn](https://github.com/royvanrijn). Essentially, it tries to optimize for the word which would yield the most information about the solution.

For future strategies, I plan to take into account factors such as the average number of possible words for each possible response. Maybe further down the line I would consider english morphology but for now I'm keeping things language-agnostic.


#### This will not help you cheat at wordle. It's just a program that plays games against itself to try to find a good strategy.

### Why Rust?
It's fast and I'm trying to learn it.
### How fast?
Currently the program can simulate about 320 games per second (on my 2 core laptop) if the initial word is pre-computed (this step takes close to a second on its own otherwise).

## Usage

```
wordle-inator

USAGE:
    wordle-inator [OPTIONS]

OPTIONS:
    -h, --help
            Print help information
            
    -i, --initial-guess <INITIAL_GUESS>
            initial word to use as a guess (e.g. if it's been precomputed)

    -m, --max-guesses <MAX_GUESSES>
            number of guesses allowed per game [default: 6]

    -p, --plays <PLAYS>
            number of games to simulate [default: 1]

    -s, --solution <SOLUTION>
            word to use as the solution (by default a random word is chosen from the word list)

    -w, --wordlist <WORDLIST>
            path to a txt file containing the list of words to use [default: 5_letter_words.txt]
```
