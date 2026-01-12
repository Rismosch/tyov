# Thousand Year Old Vampire

Random number generator and templates for the solo RPG ["Thousand Year Old Vampire" (TYOV) by Tim Hutchings](https://en.wikipedia.org/wiki/Thousand_Year_Old_Vampire).


## Ranom Number Generator (RNG)

If you want to use the RNG, you need to have [Rust](https://rust-lang.org/learn/get-started/) installed on your system.

To use the RNG, clone this repo and run it:

    git clone https://github.com/Rismosch/tyov.git
    cd tyov
    cargo run

The RNG will run forever. Press `[Ctrl] + [C]` to stop it.

While the RNG is running, press `[Enter]` to generate a new number. It does so as per the rules of TYOV: It rolls a D10 and a D6, and subtracts them from another:

> D10 - D6 = random number

## Templates

At the root of this repo you will find `template_xxx.md`. Choose the one in your desired language. They have been organized as per the rules of TYOV. The entries have been numbered from 1 to 15.

To start a new game and make a character, fill in the entry numbered 1, then 2, then 3 and so on. Once every entry is filled, start at prompt 1.