use std::collections::HashMap;

use range_set::RangeSet;

struct CharacterCategoryIdentifier(String);
enum ChracterCategoryMode {
    Block,
    Individual,
    Ignored,
}
struct CharacterCategoryInfo {
    ranges: RangeSet<[char; 2]>,
    mode: ChracterCategoryMode,
}
struct CharacterCategories(HashMap<CharacterCategoryIdentifier, CharacterCategoryInfo>);

fn main() {
    println!("Hello, world!");
}
