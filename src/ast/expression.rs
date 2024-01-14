use from_pest::{ConversionError, FromPest, Void};
use pest::iterators::Pairs;
use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::ast::semi::Semi;
use crate::utils::PrintAst;
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::ExpressionStatement))]
pub struct ExpressionStatement {
    pub expression: AstNode<Expression>,
    pub semi: Option<AstNode<Semi>>,
}

impl PrintAst for ExpressionStatement {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        result.push_str(&self.expression.print_ast(print_properties));
        if let Some(semi) = &self.semi {
            result.push_str(&semi.print_ast(print_properties));
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expression;

impl FromPest<'_> for Expression {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        if pest.peek().unwrap().as_rule() != Rule::Expression {
            return Err(ConversionError::NoMatch);
        }
        pest.next();

        Ok(Expression)
    }
}

impl PrintAst for Expression {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        result.push_str("TODO Expression");
        result
    }
}