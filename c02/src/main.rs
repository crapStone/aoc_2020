use std::io;

use lazy_static::lazy_static;
use regex::Regex;

use line_reader;

struct Password {
    first: u8,
    second: u8,
    character: char,
    password: String,
}

#[derive(Debug)]
struct Error;

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! {f, "error"}
    }
}

impl std::str::FromStr for Password {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (.*)$").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let password = caps.get(4).unwrap().as_str().to_string();
        let character = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let first: u8 = caps.get(1).unwrap().as_str().parse().unwrap();
        let second: u8 = caps.get(2).unwrap().as_str().parse().unwrap();

        Ok(Password {
            first,
            second,
            character,
            password,
        })
    }
}

impl Password {
    fn is_correct1(&self) -> bool {
        let mut passwd = self.password.clone();
        passwd.retain(|c| c == self.character);
        let len = passwd.len() as u8;

        if len >= self.first && len <= self.second {
            return true;
        }

        false
    }

    fn is_correct2(&self) -> bool {
        let mut valid = false;
        let chars: Vec<char> = self.password.chars().collect();

        if chars[(self.first - 1) as usize] == self.character {
            valid = !valid;
        }
        if chars[(self.second - 1) as usize] == self.character {
            valid = !valid;
        }

        valid
    }
}

fn main() -> io::Result<()> {
    let passwords: Vec<Password> = line_reader::convert("input")?;

    println!(
        "first: {}",
        passwords.iter().filter(|pwd| pwd.is_correct1()).count()
    );

    println!(
        "second: {}",
        passwords.iter().filter(|pwd| pwd.is_correct2()).count()
    );

    Ok(())
}
