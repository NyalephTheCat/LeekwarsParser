#[derive(Clone, Copy)]
pub struct PrintProperties {
    pub keep_comments: bool,
    pub simplify_whitespace: bool,
    pub compact: bool,
    pub indent: usize,
}

impl Default for PrintProperties {
    fn default() -> Self {
        PrintProperties {
            keep_comments: true,
            simplify_whitespace: false,
            compact: false,
            indent: 0,
        }
    }
}

pub trait PrintAst {
    fn print_ast(&self, print_properties: PrintProperties) -> String;
}