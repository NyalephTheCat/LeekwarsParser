use from_pest::{ConversionError, FromPest, Void};
use pest::iterators::Pairs;
use crate::ast::type_annotation::TypeAnnotation;
use crate::ast::identifier::Identifier;
use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::ast::block_statement::BlockStatement;
use crate::ast::semi::Semi;
use crate::utils::{find_next_non_comment_or_whitespace, PrintAst};
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDeclaration {
    pub function_keyword: AstNode<FunctionKeyword>,
    pub identifier: AstNode<Identifier>,
    pub lparen: AstNode<Lparen>,
    pub parameters: Option<AstNode<Parameters>>,
    pub rparen: AstNode<Rparen>,
    pub block_statement: AstNode<BlockStatement>,
    pub semi: Option<AstNode<Semi>>,
}

impl PrintAst for FunctionDeclaration {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        result.push_str(&self.function_keyword.print_ast(print_properties));
        result.push_str(&self.identifier.print_ast(print_properties));
        result.push_str(&self.lparen.print_ast(print_properties));
        if let Some(parameters) = &self.parameters {
            result.push_str(&parameters.print_ast(print_properties));
        }
        result.push_str(&self.rparen.print_ast(print_properties));
        result.push_str(&self.block_statement.print_ast(print_properties));
        if let Some(semi) = &self.semi {
            result.push_str(&semi.print_ast(print_properties));
        }
        result
    }
}

impl FromPest<'_> for FunctionDeclaration {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        if pest.peek().unwrap().as_rule() != Rule::FunctionDeclaration {
            return Err(ConversionError::NoMatch);
        }
        let mut context = pest.next().unwrap().into_inner();

        let function_keyword = AstNode::from_pest(&mut context)?;
        let identifier = AstNode::from_pest(&mut context)?;
        let lparen = AstNode::from_pest(&mut context)?;
        let parameters = AstNode::from_pest(&mut context).ok(); println!("parameters: {:?}", parameters);
        let rparen = AstNode::from_pest(&mut context)?; println!("rparen: {:?}", rparen);
        let block_statement = AstNode::from_pest(&mut context)?;
        let semi = AstNode::from_pest(&mut context).ok();

        Ok(FunctionDeclaration {
            function_keyword,
            identifier,
            lparen,
            parameters,
            rparen,
            block_statement,
            semi,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::Function))]
pub struct FunctionKeyword;

impl PrintAst for FunctionKeyword {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("function")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameters {
    pub parameter: Vec<AstNode<Parameter>>,
    pub comma: Vec<AstNode<Comma>>,
}

impl FromPest<'_> for Parameters {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<Self::Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        let mut parameter = Vec::new();
        let mut comma = Vec::new();
        let mut current_rule = pest.peek().ok_or(ConversionError::NoMatch)?;
        if current_rule.as_rule() != Rule::Parameters {
            return Err(ConversionError::NoMatch);
        }
        current_rule = pest.next().ok_or(ConversionError::NoMatch)?;
        let mut context = &mut current_rule.clone().into_inner();
        loop {
            let next = find_next_non_comment_or_whitespace(&mut context)?;
            if let Some(next) = next {
                match next.as_rule() {
                    Rule::Parameter => parameter.push(AstNode::from_pest(&mut context)?),
                    Rule::Comma => comma.push(AstNode::from_pest(&mut context)?),
                    _ => break,
                };
            } else {
                break;
            }
        }

        Ok(Parameters {
            parameter,
            comma,
        })
    }
}

impl PrintAst for Parameters {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        for (parameter, comma) in self.parameter.iter().zip(self.comma.iter()) {
            result.push_str(&parameter.print_ast(print_properties));
            result.push_str(&comma.print_ast(print_properties));
        }
        // Add the last parameter, which doesn't have a comma after it
        result.push_str(&self.parameter.last().unwrap().print_ast(print_properties));
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPest)]
#[pest_ast(rule(Rule::Parameter))]
pub struct Parameter {
    pub type_annotation: Option<AstNode<TypeAnnotation>>,
    pub identifier: AstNode<Identifier>,
}

impl PrintAst for Parameter {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        if let Some(type_annotation) = &self.type_annotation {
            result.push_str(&type_annotation.print_ast(print_properties));
        }
        result.push_str(&self.identifier.print_ast(print_properties));
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPest)]
#[pest_ast(rule(Rule::Comma))]
pub struct Comma;

impl PrintAst for Comma {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from(",")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPest)]
#[pest_ast(rule(Rule::LParen))]
pub struct Lparen;

impl PrintAst for Lparen {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("(")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPest)]
#[pest_ast(rule(Rule::RParen))]
pub struct Rparen;

impl PrintAst for Rparen {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from(")")
    }
}