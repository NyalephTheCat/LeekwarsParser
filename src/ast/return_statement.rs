use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::ast::expression::Expression;
use crate::ast::semi::Semi;
use crate::ast::statement::Statement;
use crate::utils::PrintAst;
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::ReturnStatement))]
pub struct ReturnStatement {
    pub return_keyword: AstNode<ReturnKeyword>,
    pub expression: Option<AstNode<Expression>>,
    pub semi: Option<AstNode<Semi>>,
}

impl PrintAst for ReturnStatement {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        result.push_str(&self.return_keyword.print_ast(print_properties));
        if let Some(expression) = &self.expression {
            result.push_str(&expression.print_ast(print_properties));
        }
        if let Some(semi) = &self.semi {
            result.push_str(&semi.print_ast(print_properties));
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::Return))]
pub struct ReturnKeyword;

impl PrintAst for ReturnKeyword {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("return")
    }
}