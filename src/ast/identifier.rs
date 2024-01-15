use pest_ast::FromPest;
use crate::utils::PrintAst;
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::Identifier))]
pub struct Identifier {
    #[pest_ast(outer(with(span_into_str)))]
    pub name: String,
}

impl PrintAst for Identifier {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        self.name.clone()
    }
}

pub fn span_into_str(span: pest::Span) -> String {
    span.as_str().to_string()
}