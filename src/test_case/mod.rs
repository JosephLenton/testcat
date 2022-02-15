mod ast;
mod grammar;
mod output;

pub use crate::common::Error;
pub use crate::common::Result;
use ::proc_macro2::TokenStream;

pub fn it_impl(stream: TokenStream) -> TokenStream {
    test_case_impl(stream, &"it")
}

pub fn test_impl(stream: TokenStream) -> TokenStream {
    test_case_impl(stream, &"test")
}

fn test_case_impl(stream: TokenStream, prefix: &'static str) -> TokenStream {
    let result = grammar::parse(stream).map(|ast| output::build(ast, prefix));

    match result {
        Ok(output) => output,
        Err(err) => panic!("{}", err),
    }
}

#[cfg(test)]
mod test_case_impl {
    use super::*;
    use ::pretty_assertions::assert_eq;
    use ::quote::quote;

    #[test]
    fn it_should_provide_test_description_and_function_for_test() {
        let output = test_impl(quote! {
          "should do blah and not foo", test_foo_blah
        });

        let expected = quote! {
          #[test]
          fn test_should_do_blah_and_not_foo() {
            test_foo_blah()
          }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }

    #[test]
    fn it_should_provide_test_description_and_function_for_it() {
        let output = it_impl(quote! {
          "should do blah and not foo", test_foo_blah
        });

        let expected = quote! {
          #[test]
          fn it_should_do_blah_and_not_foo() {
            test_foo_blah()
          }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }

    #[test]
    fn it_should_allow_module_names() {
        let output = it_impl(quote! {
          "should do blah and not foo", some_module::test_blah_not_foo
        });

        let expected = quote! {
          #[test]
          fn it_should_do_blah_and_not_foo() {
            some_module::test_blah_not_foo()
          }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }
}
