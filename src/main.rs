use std::collections::HashSet;
use std::{fs, io};

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("./1000-most-common-words.txt")?;

    let mut words: Vec<_> = contents.lines().map(|line| line.trim()).collect();

    words.sort();

    for word in words.clone() {
        println!("{word}");
    }

    let stdin = io::stdin();

    println!("This is essentially echo.");

    let mut buffer = String::new();

    let mut unique_words = HashSet::new();

    let mut previous_word = "".to_string();

    while buffer != "q" {
        println!("input: ");

        buffer = String::new();
        stdin.read_line(&mut buffer)?;
        buffer = buffer.trim().into();
        println!("{previous_word} :: {buffer}");

        let was_new_word: bool = unique_words.insert(buffer.clone());
        let is_valid: bool = isvalid(&previous_word, &buffer);

        if !was_new_word {
            println!("YOU LOSE. WORD REPEATED.");
            return Ok(());
        } else if !previous_word.is_empty() && !is_valid {
            println!("YOU LOSE. WORD NOT VALID.");
            return Ok(());
        } else {
            // can play
        }

        previous_word = buffer.clone();

        let computer_options = valid_return(&words, &previous_word);
        let op_word = computer_options.first();

        match op_word {
            Some(op_word) => {
                println!("{op_word}");
                unique_words.insert(op_word.clone());
                previous_word = op_word.clone();
            }
            None => {
                println!("YOU WIN!");
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn valid_return(wordlist: &Vec<&str>, usr_input: &String) -> Vec<String> {
    wordlist
        .iter()
        .filter(|&&word| isvalid(&usr_input, word))
        .map(|s| s.to_string())
        .collect()
}

pub fn isvalid(opword: &str, uword: &str) -> bool {
    if opword.is_empty() || uword.is_empty() {
        return false;
    }
    if opword.contains(char::is_numeric) {
        return false;
    }

    opword.ends_with(|c: char| c == uword.chars().next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_COM_WORDS: [&'static str; 6] =
        ["apple", "banana", "cherry", "date", "eggplant", "bacon"];

    #[test]
    fn return_empty_when_no_valid_word() {
        let wordlist = Vec::from(DEMO_COM_WORDS);

        let result: Vec<&str> = vec![];
        assert_eq!(valid_return(&wordlist, &String::from("zulu")), result);

        let result = vec!["eggplant".to_string()];
        assert_eq!(valid_return(&wordlist, &String::from("fate")), result);
    }

    #[test]
    fn return_valid_word_responses() {
        let wordlist = Vec::from(DEMO_COM_WORDS);
        assert_eq!(
            valid_return(&wordlist, &"glob".to_string()),
            vec!["banana".to_string(), "bacon".to_string()]
        );
        assert_eq!(
            valid_return(&wordlist, &"dad".to_string()),
            vec!["date".to_string()]
        );
    }

    #[test]
    #[ignore = "cuz"]
    fn multiple_matches() {
        let _wordlist = Vec::from(DEMO_COM_WORDS);
        todo!()
    }

    #[test]
    fn words_cant_be_blank() {
        let r = !isvalid("", "");
        assert!(r, "invalid if both blank");

        let r = !isvalid("a", "");
        assert!(r, "invalid if either blank");

        let r = !isvalid("", "a");
        assert!(r, "invalid if either blank");

        let r = isvalid("a", "a");
        assert!(r, "should have been valid");
    }

    #[test]
    fn num_not_allowed() {
        for i in 1..=9 {
            assert!(
                !isvalid(&i.to_string(), &(i + 1).to_string()),
                "Contained a number"
            );
        }
    }

    #[test]
    fn first_letter() {
        assert!(isvalid("op", "pl"));
        assert!(!isvalid("pl", "op"));
    }
}
