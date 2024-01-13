use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::ast::expression::Expression;
use crate::ast::semi::Semi;
use crate::ast::statement::Statement;
use crate::utils::PrintAst;
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::WhileStatement))]
pub struct WhileStatement {
    pub while_keyword: AstNode<WhileKeyword>,
    pub lparen: AstNode<LParen>,
    pub expression: AstNode<Expression>,
    pub rparen: AstNode<RParen>,
    pub statement: AstNode<Statement>,
}

impl PrintAst for WhileStatement {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        result.push_str(&self.while_keyword.print_ast(print_properties));
        result.push_str(&self.lparen.print_ast(print_properties));
        result.push_str(&self.expression.print_ast(print_properties));
        result.push_str(&self.rparen.print_ast(print_properties));
        result.push_str(&self.statement.print_ast(print_properties));
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::While))]
pub struct WhileKeyword;

impl PrintAst for WhileKeyword {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("while")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::LParen))]
pub struct LParen;

impl PrintAst for LParen {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("(")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::RParen))]
pub struct RParen;

impl PrintAst for RParen {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from(")")
    }
}