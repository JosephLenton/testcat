mod ast;
mod grammar;
mod output;

pub use crate::common::Error;
pub use crate::common::Result;
use ::proc_macro2::TokenStream;

pub fn describe_impl(stream: TokenStream) -> TokenStream {
    let result = grammar::parse(stream).map(output::build);

    match result {
        Ok(output) => output,
        Err(err) => panic!("{}", err),
    }
}

#[cfg(test)]
mod describe_impl {
    use super::*;
    use ::pretty_assertions::assert_eq;
    use ::quote::quote;

    #[test]
    fn it_should_provide_describe_blocks() {
        let output = describe_impl(quote! {
          "rendering example", {
            it!("should do foo", test_foo);
            it!("should do bar", test_bar);
          }
        });

        let expected = quote! {
          #[cfg(test)]
          mod rendering_example {
            use super::*;

            it!("should do foo", test_foo);
            it!("should do bar", test_bar);
          }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }

    #[test]
    fn it_should_prefix_test_modules_with_an_underscore_when_identifier_starts_with_a_number() {
        let output = describe_impl(quote! {
          "123 abc", {
            it!("should do foo", test_foo);
          }
        });

        let expected = quote! {
          #[cfg(test)]
          mod _123_abc {
            use super::*;

            it!("should do foo", test_foo);
          }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }
}
