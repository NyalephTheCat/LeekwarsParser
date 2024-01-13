use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::ast::expression::Expression;
use crate::ast::semi::Semi;
use crate::ast::statement::Statement;
use crate::ast::while_statement::{LParen, RParen, WhileKeyword};
use crate::utils::PrintAst;
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::DoWhileStatement))]
pub struct DoWhileStatement {
    pub do_keyword: AstNode<DoKeyword>,
    pub statement: AstNode<Statement>,
    pub while_keyword: AstNode<WhileKeyword>,
    pub lparen: AstNode<LParen>,
    pub expression: AstNode<Expression>,
    pub rparen: AstNode<RParen>,
    pub semi: Option<AstNode<Semi>>,
}

impl PrintAst for DoWhileStatement {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        result.push_str(&self.do_keyword.print_ast(print_properties));
        result.push_str(&self.statement.print_ast(print_properties));
        result.push_str(&self.while_keyword.print_ast(print_properties));
        result.push_str(&self.lparen.print_ast(print_properties));
        result.push_str(&self.expression.print_ast(print_properties));
        result.push_str(&self.rparen.print_ast(print_properties));
        if let Some(semi) = &self.semi {
            result.push_str(&semi.print_ast(print_properties));
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::Do))]
pub struct DoKeyword;

impl PrintAst for DoKeyword {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("do")
    }
}