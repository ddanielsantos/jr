mod token;

pub use token::Token;

pub fn is_string(c: char) -> bool {
    // TODO: #3 handle accented characters

    let specials = "_";
    ('a'..='z').contains(&c.to_ascii_lowercase()) | specials.contains(c)
}

#[test]
fn uppercase_a_is_string() {
    assert_eq!(true, is_string('A'))
}

#[test]
fn lowercase_a_is_string() {
    assert_eq!(true, is_string('a'))
}

#[test]
#[ignore]
fn accented_a_is_string() {
    assert_eq!(true, is_string('á'))
}
