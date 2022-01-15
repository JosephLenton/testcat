use crate::common::is_starting_ident_char;
use crate::describe::ast::DescribeAST;

use ::proc_macro2::Ident;
use ::proc_macro2::TokenStream;
use ::quote::format_ident;
use ::quote::quote;

pub fn build(ast: DescribeAST) -> TokenStream {
    let description_ident = build_description_ident(&ast.description);
    let code_block = &ast.code_block;

    quote! {
      #[cfg(test)]
      mod #description_ident {
          use super::*;

          #code_block
      }
    }
}

fn build_description_ident(description: &str) -> Ident {
    let c = description
        .chars()
        .into_iter()
        .next()
        .expect("Empty description given (this should never happen)");

    if is_starting_ident_char(c) {
        format_ident!("{description}")
    } else {
        format_ident!("_{description}")
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
