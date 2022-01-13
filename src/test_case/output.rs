use crate::test_case::ast::TestCaseAST;

use ::proc_macro2::TokenStream;
use ::quote::format_ident;
use ::quote::quote;

pub fn build(ast: TestCaseAST) -> TokenStream {
    let test_description = format_ident!("{}", &ast.test_description);
    let test_name = format_ident!("{}", &ast.test_name);

    quote! {
        #[test]
        fn #test_description() {
          #test_name()
        }
    }
}

#[cfg(test)]
mod build {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_output_test_function_with_wrapper() {
        let output = build(TestCaseAST {
            test_description: "it_should_do_blah".to_string(),
            test_name: "my_test_function".to_string(),
        });

        let expected = quote! {
          #[test]
          fn it_should_do_blah() {
            my_test_function()
          }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }
}
