use crate::common::chomp_comma;
use crate::common::chomp_test_description;
use crate::common::Error;
use crate::common::Result;
use crate::test_case::ast::TestCaseAST;

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

fn chomp_test_name<I>(iterator: &mut I) -> Result<String>
where
    I: Iterator<Item = TokenTree>,
{
    match iterator.next() {
        Some(TokenTree::Ident(test_name)) => Ok(test_name.to_string()),
        _ => Err(Error::TestNameExpected),
    }
}
