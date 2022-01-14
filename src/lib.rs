mod common;
mod describe;
mod test_case;

use crate::describe::describe_impl;
use crate::test_case::test_case_impl;
use ::proc_macro::TokenStream;

#[proc_macro]
pub fn test(stream: TokenStream) -> TokenStream {
    test_case_impl(stream.into()).into()
}

#[proc_macro]
pub fn it(stream: TokenStream) -> TokenStream {
    test(stream)
}

#[proc_macro]
pub fn describe(stream: TokenStream) -> TokenStream {
    describe_impl(stream.into()).into()
}
