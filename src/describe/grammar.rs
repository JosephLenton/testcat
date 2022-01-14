use crate::common::chomp_comma;
use crate::common::chomp_test_description;
use crate::common::Error;
use crate::common::Result;
use crate::describe::ast::DescribeAST;

use ::proc_macro2::Delimiter;
use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;

use ::std::iter::Iterator;

pub fn parse(stream: TokenStream) -> Result<DescribeAST> {
    let mut iterator = stream.into_iter();
    let description = chomp_test_description(&mut iterator)?;
    chomp_comma(&mut iterator)?;
    let code_block = chomp_code_block(&mut iterator)?;

    if iterator.next().is_some() {
        return Err(Error::ExcessTokensFound);
    }

    Ok(DescribeAST {
        description,
        code_block,
    })
}

fn chomp_code_block<I>(iterator: &mut I) -> Result<TokenStream>
where
    I: Iterator<Item = TokenTree>,
{
    match iterator.next() {
        Some(TokenTree::Group(group)) => {
            if group.delimiter() != Delimiter::Brace {
                return Err(Error::CodeBlockExpected);
            }

            Ok(group.stream())
        }
        _ => Err(Error::CodeBlockExpected),
    }
}
