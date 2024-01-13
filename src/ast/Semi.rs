use pest_ast::FromPest;
use crate::utils::PrintAst;
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::Semi))]
pub struct Semi;

impl PrintAst for Semi {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        String::from(";")
    }
}