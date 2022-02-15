use crate::common::chomp_comma;
use crate::common::chomp_test_description;
use crate::common::Result;
use crate::test_case::ast::TestCaseAST;

use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;
use ::std::iter::Iterator;
use ::syn::Path;

pub fn parse(stream: TokenStream) -> Result<TestCaseAST> {
    let mut iterator = stream.into_iter();
    let test_description = chomp_test_description(&mut iterator)?;
    chomp_comma(&mut iterator)?;
    let test_name = chomp_test_name(iterator)?;

    Ok(TestCaseAST {
        test_name,
        test_description,
    })
}

fn chomp_test_name<I>(iterator: I) -> Result<Path>
where
    I: Iterator<Item = TokenTree>,
{
    let mut token_stream = TokenStream::new();
    token_stream.extend(iterator);

    let name: Path = ::syn::parse2(token_stream)?;
    Ok(name)
}
