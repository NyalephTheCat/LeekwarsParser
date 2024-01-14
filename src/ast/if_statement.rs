use pest::iterators::Pairs;
use from_pest::{ConversionError, Void};
use from_pest::FromPest;
use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::ast::expression::Expression;
use crate::ast::semi::Semi;
use crate::ast::statement::Statement;
use crate::ast::while_statement::{LParen, RParen, WhileKeyword};
use crate::utils::PrintAst;
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfStatement {
    pub if_keyword: AstNode<IfKeyword>,
    pub lparen: AstNode<LParen>,
    pub expression: AstNode<Expression>,
    pub rparen: AstNode<RParen>,
    pub statement: AstNode<Statement>,
    pub else_keyword: Option<AstNode<ElseKeyword>>,
    pub else_statement: Option<AstNode<Statement>>,
    pub semi: Option<AstNode<Semi>>,
}

impl FromPest<'_> for IfStatement {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<Self::Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        let mut current_rule = pest.peek().ok_or(ConversionError::NoMatch)?;
        if current_rule.as_rule() != Rule::IfStatement {
            return Err(ConversionError::NoMatch);
        }
        current_rule = pest.next().ok_or(ConversionError::NoMatch)?;

        let mut context = &mut current_rule.clone().into_inner();

        let if_keyword = AstNode::from_pest(&mut context)?;
        let lparen = AstNode::from_pest(&mut context)?;
        let expression = AstNode::from_pest(&mut context)?;
        let rparen = AstNode::from_pest(&mut context)?;
        let statement = AstNode::from_pest(&mut context)?;
        let else_keyword = AstNode::from_pest(&mut context).ok();
        let else_statement = AstNode::from_pest(&mut context).ok();
        let semi = AstNode::from_pest(&mut context).ok();

        Ok(IfStatement {
            if_keyword,
            lparen,
            expression,
            rparen,
            statement,
            else_keyword,
            else_statement,
            semi,
        })
    }
}

impl PrintAst for IfStatement {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        result.push_str(&self.if_keyword.print_ast(print_properties));
        result.push_str(&self.lparen.print_ast(print_properties));
        result.push_str(&self.expression.print_ast(print_properties));
        result.push_str(&self.rparen.print_ast(print_properties));
        result.push_str(&self.statement.print_ast(print_properties));
        if let Some(else_keyword) = &self.else_keyword {
            result.push_str(&else_keyword.print_ast(print_properties));
        }
        if let Some(else_statement) = &self.else_statement {
            result.push_str(&else_statement.print_ast(print_properties));
        }
        if let Some(semi) = &self.semi {
            result.push_str(&semi.print_ast(print_properties));
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::If))]
pub struct IfKeyword;

impl PrintAst for IfKeyword {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("if")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::Else))]
pub struct ElseKeyword;

impl PrintAst for ElseKeyword {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("else")
    }
}