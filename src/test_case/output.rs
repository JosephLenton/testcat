use crate::test_case::ast::TestCaseAST;

use ::proc_macro2::TokenStream;
use ::quote::format_ident;
use ::quote::quote;

pub fn build(ast: TestCaseAST, test_description_prefix: &'static str) -> TokenStream {
    let test_description = &ast.test_description;
    let test_description_ident = format_ident!("{test_description_prefix}_{test_description}");
    let test_name_ident = &ast.test_name;

    quote! {
        #[test]
        fn #test_description_ident() {
          #test_name_ident()
        }
    }
}
