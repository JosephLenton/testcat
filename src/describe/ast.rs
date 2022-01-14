use ::proc_macro2::TokenStream;

pub struct DescribeAST {
    pub description: String,
    pub code_block: TokenStream,
}
