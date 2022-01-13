use crate::test_case::ast::TestCaseAST;
use crate::test_case::Error;
use crate::test_case::Result;

use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;
use ::std::iter::Iterator;

pub fn parse(stream: TokenStream) -> Result<TestCaseAST> {
    let mut iterator = stream.into_iter();
    let test_description = chomp_test_description(&mut iterator)?;
    chomp_comma(&mut iterator)?;
    let test_name = chomp_test_name(&mut iterator)?;

    if iterator.next().is_some() {
        return Err(Error::ExcessTokensFound);
    }

    Ok(TestCaseAST {
        test_name,
        test_description,
    })
}

fn chomp_test_description<I>(iterator: &mut I) -> Result<String>
where
    I: Iterator<Item = TokenTree>,
{
    let mut test_description = match iterator.next() {
        Some(TokenTree::Literal(test_description)) => Ok(test_description.to_string()),
        _ => Err(Error::TestDescriptionExpected),
    }?;

    if test_description.starts_with('"') {
        test_description = test_description.as_str()[1..test_description.len() - 1].to_string();
    }

    test_description_to_test_name(&mut test_description);
    if test_description.len() == 0 {
        return Err(Error::EmptyDescriptionGiven);
    }

    Ok(test_description)
}

fn test_description_to_test_name(description: &mut String) {
    for i in 0..description.len() {
        if description.get(i..i + 1) == Some(" ") {
            description.replace_range(i..i + 1, "_");
        }
    }
}

fn chomp_comma<I>(iterator: &mut I) -> Result<()>
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

fn chomp_test_name<I>(iterator: &mut I) -> Result<String>
where
    I: Iterator<Item = TokenTree>,
{
    match iterator.next() {
        Some(TokenTree::Ident(test_name)) => Ok(test_name.to_string()),
        _ => Err(Error::TestNameExpected),
    }
}
