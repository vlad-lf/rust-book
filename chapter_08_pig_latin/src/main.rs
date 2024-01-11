use std::{io, collections::HashMap};

#[derive(Debug)]
enum LetterTypes {
    Vowel,
    Consonant
}

fn main() {
    println!("Welcome to the words pig latinizer!");

    println!("Please, enter the string you'd like to translate (latin characters separated by spaces only)");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let words: Vec<&str> = input.split(' ').collect();

    // println!("You've entered {:?}", words);

    let mut res = String::new();

    let mut vowels: HashMap<char, LetterTypes> = HashMap::new();
    vowels.insert('a', LetterTypes::Vowel);
    vowels.insert('e', LetterTypes::Vowel);
    vowels.insert('i', LetterTypes::Vowel);
    vowels.insert('o', LetterTypes::Vowel);
    vowels.insert('u', LetterTypes::Vowel);
    

    for word in words {
        let word = word.trim();
        let chars: Vec<char> = word.to_lowercase().chars().collect();
        match chars.first() {
            Some(c) => {
                let letter_type = vowels.get(c).unwrap_or(&LetterTypes::Consonant);
                // println!("The first char of world '{word}' is '{c}' and it is {:?}", letter_type);
                match letter_type {
                    LetterTypes::Vowel => res = res + word + "-hay ",
                    LetterTypes::Consonant => res = res + &word[1..] + "-" + &word[..1] + "ay ",
                };
            },
            None => ()
        }
    }

    println!("The result is {}", res)
}
