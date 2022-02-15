use ::syn::Path;

pub struct TestCaseAST {
    pub test_description: String,
    pub test_name: Path,
}
