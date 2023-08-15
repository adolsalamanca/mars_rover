use std::eprintln;
use std::io::BufRead;
use std::num::ParseIntError;
use std::process::exit;

#[derive(Debug, PartialEq)]
pub enum Errors {
    InvalidInput,
}
pub fn read_number<R>(mut reader: R) -> Result<u64, Errors>
where
    R: BufRead
{
    let mut str = String::new();
    match reader.read_line(&mut str){
        Ok(number) => number,
        Err(_) => {
            eprintln!("error: cannot read the line");
            exit(-1);
        }
    };

    let number:Result<u64, ParseIntError> = str.trim().parse();
    if number.is_err() {
        return Err(Errors::InvalidInput)
    }

    Ok(number.unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_integer_number_should_succeed() {
        let valid_input = b"3";

        let n = read_number(&valid_input[..]);

        assert_eq!(Ok(3), n);
    }

    #[test]
    fn read_invalid_character_should_error() {
        let invalid_input = b"x";

        let n = read_number(&invalid_input[..]);

        assert_eq!(Err(Errors::InvalidInput), n);
    }

    #[test]
    fn read_negative_number_should_error() {
        let invalid_input = b"-3";

        let n = read_number(&invalid_input[..]);

        assert_eq!(Err(Errors::InvalidInput), n);
    }
}