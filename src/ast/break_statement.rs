use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::ast::semi::Semi;
use crate::utils::PrintAst;
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::BreakStatement))]
pub struct BreakStatement {
    pub break_keyword: AstNode<BreakKeyword>,
    pub semi: Option<AstNode<Semi>>,
}

impl PrintAst for BreakStatement {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        result.push_str(&self.break_keyword.print_ast(print_properties));
        if let Some(semi) = &self.semi {
            result.push_str(&semi.print_ast(print_properties));
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::Break))]
pub struct BreakKeyword;

impl PrintAst for BreakKeyword {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("break")
    }
}