use crate::domain::space::Movement;
use std::io::BufRead;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum Errors {
    InvalidInput,
    InvalidCommand,
}
pub fn read_number<R>(mut reader: R) -> Result<u64, Errors>
where
    R: BufRead,
{
    let mut str = String::new();
    let result = reader.read_line(&mut str);
    if result.is_err() {
        return Err(Errors::InvalidInput);
    }

    let number: Result<u64, ParseIntError> = str.trim().parse();
    if number.is_err() {
        return Err(Errors::InvalidInput);
    }

    Ok(number.unwrap())
}
pub fn read_commands<R>(mut reader: R) -> Result<Vec<Movement>, Errors>
where
    R: BufRead,
{
    let mut str = String::new();
    let result = reader.read_line(&mut str);
    if result.is_err() {
        return Err(Errors::InvalidInput);
    }

    let commands: Result<Vec<Movement>, Errors> = str
        .trim()
        .split(',')
        .map(|c| match c {
            "F" => Ok(Movement::F),
            "f" => Ok(Movement::F),
            "B" => Ok(Movement::B),
            "b" => Ok(Movement::B),
            "L" => Ok(Movement::L),
            "l" => Ok(Movement::L),
            "r" => Ok(Movement::R),
            "R" => Ok(Movement::R),
            _ => Err(Errors::InvalidCommand),
        })
        .collect();

    commands
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::input::Errors::InvalidCommand;

    #[test]
    fn should_read_commands_properly_if_all_are_valid() {
        let valid_input = b"F,B,L,R";

        let n = read_commands(&valid_input[..]);
        let expected_commands: Vec<Movement> =
            vec![Movement::F, Movement::B, Movement::L, Movement::R];

        assert_eq!(Ok(expected_commands), n);
    }

    #[test]
    fn should_fail_after_reading_commands_that_include_one_invalid() {
        let valid_input = b"F,B,X,R";

        let n = read_commands(&valid_input[..]);

        assert_eq!(Err(InvalidCommand), n);
    }

    #[test]
    fn should_read_number_properly_if_is_an_integer() {
        let valid_input = b"3";

        let n = read_number(&valid_input[..]);

        assert_eq!(Ok(3), n);
    }

    #[test]
    fn should_fail_reading_number_if_is_a_letter() {
        let invalid_input = b"x";

        let n = read_number(&invalid_input[..]);

        assert_eq!(Err(Errors::InvalidInput), n);
    }

    #[test]
    fn should_fail_reading_number_if_is_a_negative_number() {
        let invalid_input = b"-3";

        let n = read_number(&invalid_input[..]);

        assert_eq!(Err(Errors::InvalidInput), n);
    }
}
