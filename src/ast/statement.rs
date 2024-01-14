use crate::ast::expression::ExpressionStatement;
use from_pest::{ConversionError, FromPest, Void};
use pest::iterators::Pairs;
use crate::ast::break_statement::BreakStatement;
use crate::ast::do_while_statement::DoWhileStatement;
use crate::ast::return_statement::ReturnStatement;
use crate::ast::while_statement::WhileStatement;
use crate::ast::if_statement::IfStatement;
use super::AstNode;
use super::block_statement::BlockStatement;
use super::continue_statement::ContinueStatement;
use super::semi::Semi;
use crate::lsv4::Rule;
use crate::utils::{find_next_non_comment_or_whitespace, PrintAst};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    EmptyStatement(AstNode<Semi>),
    BlockStatement(AstNode<BlockStatement>),
    ReturnStatement(AstNode<ReturnStatement>),
    BreakStatement(AstNode<BreakStatement>),
    ContinueStatement(AstNode<ContinueStatement>),
    ExpressionStatement(AstNode<ExpressionStatement>),
    WhileStatement(AstNode<WhileStatement>),
    DoWhileStatement(AstNode<DoWhileStatement>),
    IfStatement(AstNode<IfStatement>),
}

impl PrintAst for Statement {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        match self {
            Statement::EmptyStatement(semi) => semi.print_ast(print_properties),
            Statement::BlockStatement(block_statement) => block_statement.print_ast(print_properties),
            Statement::ReturnStatement(return_statement) => return_statement.print_ast(print_properties),
            Statement::BreakStatement(break_statement) => break_statement.print_ast(print_properties),
            Statement::ContinueStatement(continue_statement) => continue_statement.print_ast(print_properties),
            Statement::ExpressionStatement(expression_statement) => expression_statement.print_ast(print_properties),
            Statement::WhileStatement(while_statement) => while_statement.print_ast(print_properties),
            Statement::DoWhileStatement(do_while_statement) => do_while_statement.print_ast(print_properties),
            Statement::IfStatement(if_statement) => if_statement.print_ast(print_properties),
        }
    }
}

impl FromPest<'_> for Statement {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<Self::Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        let mut current_rule = pest.peek().ok_or(ConversionError::NoMatch)?;
        if current_rule.as_rule() != Rule::Statement {
            return Err(ConversionError::NoMatch);
        }
        current_rule = pest.next().ok_or(ConversionError::NoMatch)?;

        let mut context = &mut current_rule.clone().into_inner();

        let next = find_next_non_comment_or_whitespace(&mut context)?;
        if let Some(next) = next {
            match next.as_rule() {
                Rule::Semi => {
                    Ok(Statement::EmptyStatement(AstNode::from_pest(&mut context)?))
                },
                Rule::BlockStatement => {
                    Ok(Statement::BlockStatement(AstNode::from_pest(&mut context)?))
                },
                Rule::ReturnStatement => {
                    Ok(Statement::ReturnStatement(AstNode::from_pest(&mut context)?))
                },
                Rule::BreakStatement => {
                    Ok(Statement::BreakStatement(AstNode::from_pest(&mut context)?))
                },
                Rule::ContinueStatement => {
                    Ok(Statement::ContinueStatement(AstNode::from_pest(&mut context)?))
                },
                Rule::ExpressionStatement => {
                    Ok(Statement::ExpressionStatement(AstNode::from_pest(&mut context)?))
                },
                Rule::WhileStatement => {
                    Ok(Statement::WhileStatement(AstNode::from_pest(&mut context)?))
                },
                Rule::DoWhileStatement => {
                    Ok(Statement::DoWhileStatement(AstNode::from_pest(&mut context)?))
                },
                Rule::IfStatement => {
                    Ok(Statement::IfStatement(AstNode::from_pest(&mut context)?))
                },
                rule => {
                    println!("Unexpected rule: {:?}", rule);
                    Err(ConversionError::NoMatch)
                },
            }
        } else {
            Err(ConversionError::NoMatch)
        }
    }
}