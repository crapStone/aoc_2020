use std::io::{self, BufRead};
use std::path::Path;
use std::{fmt::Debug, fs::File, str::FromStr};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn convert<T, P>(filename: P) -> io::Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    Ok(lines.map(|s| s.unwrap().parse().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_conversion_string() {
        let strings: Vec<String> = convert("test_input").unwrap();

        assert_eq!(
            strings,
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        );
    }

    #[test]
    fn test_conversion_u8() {
        let numbers: Vec<u8> = convert("test_input").unwrap();

        assert_eq!(numbers, vec![1, 2, 3]);
    }
}
