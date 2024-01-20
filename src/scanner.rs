use std::ops::RangeInclusive;

use rangemap::RangeInclusiveMap;

#[derive(Debug)]
pub enum Error {
    RangesIntersect {
        existing: (RangeInclusive<char>, Category),
        new: (RangeInclusive<char>, Category),
    },
    UnknownCharacter(char),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CategoryMode {
    Block,
    Individual,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Category {
    name: String,
    mode: CategoryMode,
}
impl Category {
    pub fn block(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mode: CategoryMode::Block,
        }
    }
    pub fn individual(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mode: CategoryMode::Individual,
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    content: &'a str,
    category: Category,
}

pub struct CharacterCategories(RangeInclusiveMap<u32, Category>);
impl CharacterCategories {
    pub fn new() -> Self {
        Self(RangeInclusiveMap::new())
    }
    pub fn insert_range(
        &mut self,
        range: RangeInclusive<char>,
        info: &Category,
    ) -> Result<(), Error> {
        let range_int = *range.start() as u32..=*range.end() as u32;
        if let Some((overlap_range, overlap_info)) = self.0.overlapping(&range_int).next() {
            return Err(Error::RangesIntersect {
                existing: (
                    char::from_u32(*overlap_range.start()).unwrap()
                        ..=char::from_u32(*overlap_range.end()).unwrap(),
                    overlap_info.to_owned(),
                ),
                new: (range, info.clone()),
            });
        }
        self.0.insert(range_int, info.clone());
        Ok(())
    }

    pub fn insert(&mut self, c: char, info: &Category) -> Result<(), Error> {
        self.insert_range(c..=c, info)
    }
    pub fn insert_many(
        &mut self,
        chars: impl IntoIterator<Item = char>,
        info: &Category,
    ) -> Result<(), Error> {
        chars.into_iter().try_for_each(|c| self.insert(c, info))
    }

    pub fn extract_token<'a>(&self, input: &mut &'a str) -> Option<Result<Token<'a>, Error>> {
        let first_char = match input.chars().next() {
            Some(c) => c,
            None => return None,
        };
        let Some(category) = self.0.get(&(first_char as u32)) else {
            return Some(Err(Error::UnknownCharacter(first_char)));
        };
        let end_index = match category.mode {
            CategoryMode::Individual => first_char.len_utf8(),
            CategoryMode::Block => input
                .chars()
                .take_while(|&ch| self.0.get(&(ch as u32)) == Some(category))
                .map(|ch| ch.len_utf8())
                .sum(),
        };
        let token = Token {
            content: &input[..end_index],
            category: category.clone(),
        };
        *input = &input[end_index..];
        Some(Ok(token))
    }
}
