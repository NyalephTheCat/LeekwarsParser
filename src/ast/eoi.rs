use pest_ast::FromPest;
use crate::utils::PrintAst;
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::EOI))]
pub struct Eoi;

impl PrintAst for Eoi {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::new()
    }
}