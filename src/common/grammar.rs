use crate::common::Error;
use crate::common::Result;

use ::proc_macro2::TokenTree;
use ::std::iter::Iterator;

pub fn chomp_test_description<I>(iterator: &mut I) -> Result<String>
where
    I: Iterator<Item = TokenTree>,
{
    let mut test_description = match iterator.next() {
        Some(TokenTree::Literal(test_description)) => Ok(test_description.to_string()),
        _ => Err(Error::TestDescriptionExpected),
    }?;

    test_description = description_to_identifier(test_description);
    if test_description.len() == 0 {
        return Err(Error::EmptyDescriptionGiven);
    }

    Ok(test_description)
}

pub fn chomp_comma<I>(iterator: &mut I) -> Result<()>
where
    I: Iterator<Item = TokenTree>,
{
    match iterator.next() {
        Some(TokenTree::Punct(maybe_comma)) => {
            if maybe_comma.as_char() == ',' {
                return Ok(());
            }

            Err(Error::CommaExpected)
        }
        _ => Err(Error::CommaExpected),
    }
}

fn description_to_identifier(description: String) -> String {
    let mut chars = description.chars().into_iter();
    let mut dest = String::with_capacity(description.len());
    let mut is_at_start = true;

    while let Some(c) = chars.next() {
        // This first 'is_at_start' block allows us to trim left whitespace.
        if is_at_start {
            if is_non_starting_ident_char(c) {
                dest.push(c);
                is_at_start = false;
            }

            continue;
        }

        if is_non_starting_ident_char(c) {
            dest.push(c);
            continue;
        }

        if c.is_whitespace() {
            dest.push('_');
            continue;
        }
    }

    dest
}

pub fn is_starting_ident_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub fn is_non_starting_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod description_to_identifier {
    use super::*;

    #[test]
    fn it_should_return_an_empty_string_if_given_string_is_empty() {
        let result = description_to_identifier("".to_string());
        assert_eq!(result, "");
    }

    #[test]
    fn it_should_return_same_identifier_if_all_valid() {
        let result = description_to_identifier("foo".to_string());
        assert_eq!(result, "foo");
    }

    #[test]
    fn it_should_turn_spaces_into_underscores() {
        let result = description_to_identifier("foo bar".to_string());
        assert_eq!(result, "foo_bar");
    }

    #[test]
    fn it_should_trim_a_starting_quote() {
        let result = description_to_identifier("\"foo".to_string());
        assert_eq!(result, "foo");
    }

    #[test]
    fn it_should_trim_an_ending_quote() {
        let result = description_to_identifier("foo\"".to_string());
        assert_eq!(result, "foo");
    }

    #[test]
    fn it_should_trim_start_and_ending_quotes() {
        let result = description_to_identifier("\"foo\"".to_string());
        assert_eq!(result, "foo");
    }

    #[test]
    fn it_should_trim_starting_whitespace() {
        let result = description_to_identifier("  foo".to_string());
        assert_eq!(result, "foo");
    }

    #[test]
    fn it_should_drop_non_alphanumeric_characters() {
        let result = description_to_identifier("doesn't".to_string());
        assert_eq!(result, "doesnt");
    }
}
