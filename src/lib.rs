mod common;
mod describe;
mod test_case;

use ::proc_macro::TokenStream;

use crate::describe::describe_impl;
use crate::test_case::it_impl;
use crate::test_case::test_impl;

#[proc_macro]
pub fn test(stream: TokenStream) -> TokenStream {
    test_impl(stream.into()).into()
}

#[proc_macro]
pub fn it(stream: TokenStream) -> TokenStream {
    it_impl(stream.into()).into()
}

#[proc_macro]
pub fn describe(stream: TokenStream) -> TokenStream {
    describe_impl(stream.into()).into()
}
