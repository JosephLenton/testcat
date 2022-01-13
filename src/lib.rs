mod test_case;

use ::proc_macro::TokenStream;
use test_case::test_case_impl;

#[proc_macro]
pub fn test(stream: TokenStream) -> TokenStream {
    test_case_impl(stream.into()).into()
}

#[proc_macro]
pub fn it(stream: TokenStream) -> TokenStream {
    test(stream)
}
