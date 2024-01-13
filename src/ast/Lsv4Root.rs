use from_pest::{ConversionError, FromPest, Void};
use pest::iterators::Pairs;
use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::lsv4::Rule;
use crate::utils::PrintAst;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lsv4Root {
    pub statements: Vec<AstNode<Statement>>,
    pub eoi: AstNode<Eoi>,
}

impl FromPest<'_> for Lsv4Root {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<Self::Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        println!("Lsv4Root::from_pest");
        if pest.peek().unwrap().as_rule() != Rule::lsv4_root {
            return Err(ConversionError::NoMatch);
        }

        // Get inner pairs
        let mut inner_pairs = pest.next().unwrap().into_inner();
        let mut statements = Vec::new();
        while let Some(statement) = inner_pairs.next() {
            println!("statement: {:?}", statement);

            let statement = AstNode::<Statement>::from_pest(&mut Pairs::single(statement));
            if let Ok(statement) = statement {
                statements.push(statement);
            } else {
                break;
            }
        }

        Ok(Lsv4Root {
            statements,
            eoi: AstNode::<Eoi>::from_pest(pest)?,
        })
    }
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

#[derive(FromPest, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[pest_ast(rule(Rule::EOI))]
pub struct Eoi;

impl PrintAst for Eoi {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        String::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Statement; // TODO: Implement this

impl FromPest<'_> for Statement {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<Self::Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        println!("Statement::from_pest");
        if pest.peek().unwrap().as_rule() != Rule::Statement {
            return Err(ConversionError::NoMatch);
        }

        pest.next(); // Eat the Statement pair

        Ok(Statement)
    }

}

impl PrintAst for Statement {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        String::new()
    }
}