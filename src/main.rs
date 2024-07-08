use std::{fs, io};

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("./1000-most-common-words.txt")?;

    let mut words: Vec<_> = contents.lines().map(|line| line.trim()).collect();

    words.sort();

    for word in words {
        println!("{word}");
    }

    let stdin = io::stdin();

    println!("This is essentially echo.");

    let mut buffer = String::new();

    while buffer != "q" {
        println!("input: ");

        buffer = String::new();
        stdin.read_line(&mut buffer)?;
        buffer = buffer.trim().into();
        println!("{buffer:?}");
    }

    Ok(())
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
