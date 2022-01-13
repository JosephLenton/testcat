mod ast;
mod error;
mod grammar;
mod output;

pub use self::error::*;
use ::proc_macro2::TokenStream;

pub fn test_case_impl(stream: TokenStream) -> TokenStream {
    match build_test_case(stream) {
        Ok(output) => output,
        Err(err) => panic!("{}", err),
    }
}

fn build_test_case(stream: TokenStream) -> Result<TokenStream> {
    grammar::parse(stream).map(output::build)
}

#[cfg(test)]
mod test_case_impl {
    use super::*;
    use ::pretty_assertions::assert_eq;
    use ::quote::quote;

    #[test]
    fn it_should_provide_test_description_and_function() {
        let output = test_case_impl(quote! {
          "it should do blah and not foo", test_foo_blah
        });

        let expected = quote! {
          #[test]
          fn it_should_do_blah_and_not_foo() {
            test_foo_blah()
          }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }
}
