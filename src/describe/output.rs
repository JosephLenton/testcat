use crate::describe::ast::DescribeAST;

use ::proc_macro2::TokenStream;
use ::quote::format_ident;
use ::quote::quote;

pub fn build(ast: DescribeAST) -> TokenStream {
    let test_description = format_ident!("{}", &ast.description);
    let code_block = &ast.code_block;

    quote! {
      #[cfg(test)]
      mod #test_description {
          use super::*;

          #code_block
      }
    }
}

#[cfg(test)]
mod build {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_output_test_function_with_wrapper() {
        let output = build(DescribeAST {
            description: "it_should_do_blah".to_string(),
            code_block: quote! {
              do_foo();
              do_blah();
            },
        });

        let expected = quote! {
          #[cfg(test)]
          mod it_should_do_blah {
            use super::*;

            do_foo();
            do_blah();
          }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }
}
