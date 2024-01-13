use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::ast::semi::Semi;
use crate::utils::PrintAst;
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::ContinueStatement))]
pub struct ContinueStatement {
    pub continue_keyword: AstNode<ContinueKeyword>,
    pub semi: Option<AstNode<Semi>>,
}

impl PrintAst for ContinueStatement {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        result.push_str(&self.continue_keyword.print_ast(print_properties));
        if let Some(semi) = &self.semi {
            result.push_str(&semi.print_ast(print_properties));
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::Continue))]
pub struct ContinueKeyword;

impl PrintAst for ContinueKeyword {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("continue")
    }
}