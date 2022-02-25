use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number { x: i32 },
}

impl Display for FizzBuzz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FizzBuzz::FizzBuzz => f.write_str("FizzBuzz"),
            FizzBuzz::Fizz => f.write_str("Fizz"),
            FizzBuzz::Buzz => f.write_str("Buzz"),
            FizzBuzz::Number { x } => f.write_str(&x.to_string()),
        }
    }
}

impl From<i32> for FizzBuzz {
    fn from(i: i32) -> Self {
        match (i % 3, i % 5) {
            (0, 0) => FizzBuzz::FizzBuzz,
            (0, _) => FizzBuzz::Fizz,
            (_, 0) => FizzBuzz::Buzz,
            _ => FizzBuzz::Number { x: i },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_1_number() {
        assert_eq!(FizzBuzz::Number { x: 1 }, FizzBuzz::from(1))
    }

    #[test]
    fn from_3_fizz() {
        assert_eq!(FizzBuzz::Fizz, FizzBuzz::from(3))
    }

    #[test]
    fn from_5_buzz() {
        assert_eq!(FizzBuzz::Buzz, FizzBuzz::from(5))
    }

    #[test]
    fn from_15_fizzbuzz() {
        assert_eq!(FizzBuzz::FizzBuzz, FizzBuzz::from(15))
    }

    #[test]
    fn display_1() {
        assert_eq!("1", FizzBuzz::Number { x: 1 }.to_string())
    }

    #[test]
    fn display_fizz() {
        assert_eq!("Fizz", FizzBuzz::Fizz.to_string())
    }

    #[test]
    fn display_buzz() {
        assert_eq!("Buzz", FizzBuzz::Buzz.to_string())
    }

    #[test]
    fn display_fizzbuzz() {
        assert_eq!("FizzBuzz", FizzBuzz::FizzBuzz.to_string())
    }
}
