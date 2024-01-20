#![feature(try_blocks)]

mod scanner;
use scanner::{Category, CharacterCategories, Error};

fn main() {
    let result: Result<(), Error> = try {
        let mut scanner = CharacterCategories::new();
        let identifiers = Category::block("identifiers");
        scanner.insert_range('A'..='z', &identifiers)?;
        scanner.insert_range('A'..='Z', &identifiers)?;
        scanner.insert_range('0'..='9', &identifiers)?;
        let spaces = Category::individual("spaces");
        scanner.insert(' ', &spaces)?;
        scanner.insert('\n', &spaces)?;
        let operators = Category::block("operators");
        scanner.insert_many("~!@#$%^&*-+=;:,.?/|\\".chars(), &operators)?;

        let mut input = "test + 3//3a";
        let mut tokens = std::iter::from_fn(|| scanner.extract_token(&mut input));
        tokens.try_for_each(|token_res| token_res.map(|token| println!("{:?}", token)))?;
    };

    match result {
        Ok(()) => println!("DONE"),
        Err(err) => eprintln!("ERROR: {err:?}"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
