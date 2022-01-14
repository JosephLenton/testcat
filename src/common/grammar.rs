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

fn description_to_identifier(mut description: String) -> String {
    if description.starts_with('"') {
        description = description.as_str()[1..description.len() - 1].to_string();
    }

    for i in 0..description.len() {
        if description.get(i..i + 1) == Some(" ") {
            description.replace_range(i..i + 1, "_");
        }
    }

    if description.len() == 0 {
        return description;
    }

    let first_char = description.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        description = format!("_{description}");
    }

    description
}
