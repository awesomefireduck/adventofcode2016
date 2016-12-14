use std::vec::Vec;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {

    let input = File::open("input").expect("oeuaeouoeauoeuaeo");
    let mut lined_input = BufReader::new(input).lines().peekable();
    let mut message_repeats : Vec<Vec<char>> = vec![];

    while let Some(line) = lined_input.next() {
        let a : Vec<char> = line.expect("").chars().collect();
        message_repeats.push(a);
    }

    let mut counts: [HashMap<char, usize>; 8] = [HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new()];
    for repeat in message_repeats {
        for (position, character) in repeat.iter().enumerate() {
            let char_count = counts[position].entry(*character).or_insert(0);
            *char_count += 1;
        }
    }
    let mut message = String::new();
    for character_counts in counts.iter() {
        let character = find_most_occurring_character(&character_counts);
        message.push(character);
    }
    println!("{}", message);
}

fn find_most_occurring_character(character_counts: &HashMap<char, usize>) -> char {
    let mut least_occurring_char = 0 as char;
    let mut min_count = std::usize::MAX;
    for (&character, &count) in character_counts.iter() {
        if min_count > count {
            min_count = count;
            least_occurring_char = character;
        }
    }
    least_occurring_char
}

