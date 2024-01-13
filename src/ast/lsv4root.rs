use from_pest::{ConversionError, FromPest, Void};
use pest::iterators::Pairs;
use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::ast::block_statement::BlockStatement;
use crate::ast::eoi::Eoi;
use crate::ast::semi::Semi;
use crate::lsv4::Rule;
use crate::utils::{find_next_non_comment_or_whitespace, PrintAst};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lsv4Root {
    pub statements: Vec<AstNode<Statement>>,
    pub eoi: AstNode<Eoi>,
}

impl PrintAst for Lsv4Root {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        for statement in &self.statements {
            result.push_str(&statement.print_ast(print_properties));
        }
        result.push_str(&self.eoi.print_ast(print_properties));
        result
    }
}

impl FromPest<'_> for Lsv4Root {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<Self::Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        let mut current_rule = pest.peek().ok_or(ConversionError::NoMatch)?;
        if current_rule.as_rule() != Rule::lsv4_root {
            return Err(ConversionError::NoMatch);
        }
        current_rule = pest.next().ok_or(ConversionError::NoMatch)?;

        let mut statements = Vec::new();

        let mut context = &mut current_rule.clone().into_inner();

        // While you can convert to a statement, do so
        loop {
            let statement = AstNode::<Statement>::from_pest(&mut context);
            if let Ok(statement) = statement {
                statements.push(statement);
            } else {
                break;
            }
        }

        // Try to get eoi
        let eoi = Some(AstNode::<Eoi>::from_pest(&mut context)?);

        Ok(Lsv4Root {
            statements,
            eoi: eoi.ok_or(ConversionError::NoMatch)?
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    EmptyStatement(AstNode<Semi>),
    BlockStatement(AstNode<BlockStatement>),
}

impl PrintAst for Statement {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        match self {
            Statement::EmptyStatement(semi) => semi.print_ast(print_properties),
            Statement::BlockStatement(block_statement) => block_statement.print_ast(print_properties),
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
                    let semi = AstNode::<Semi>::from_pest(&mut context)?;
                    Ok(Statement::EmptyStatement(semi))
                },
                Rule::BlockStatement => {
                    let block_statement = AstNode::<BlockStatement>::from_pest(&mut context)?;
                    Ok(Statement::BlockStatement(block_statement))
                },
                _ => Err(ConversionError::NoMatch),
            }
        } else {
            Err(ConversionError::NoMatch)
        }
    }
}