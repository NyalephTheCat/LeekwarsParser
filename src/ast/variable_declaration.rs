use from_pest::{ConversionError, FromPest, Void};
use pest::iterators::Pairs;
use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::ast::continue_statement::ContinueKeyword;
use crate::ast::expression::Expression;
use crate::ast::identifier::Identifier;
use crate::ast::semi::Semi;
use crate::ast::type_annotation::TypeAnnotation;
use crate::utils::{find_next_non_comment_or_whitespace, PrintAst};
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableDeclaration {
    pub var_decl_keyword: AstNode<VarDeclKeyword>,
    pub identifier: AstNode<Identifier>,
    pub equal: Option<AstNode<Equal>>,
    pub expression: Option<AstNode<Expression>>,
    pub semi: Option<AstNode<Semi>>,
}

impl FromPest<'_> for VariableDeclaration {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<'_, Self::Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        if pest.peek().unwrap().as_rule() != Rule::VariableDeclaration {
            return Err(ConversionError::NoMatch);
        }
        let mut context = pest.next().unwrap().into_inner();

        let var_decl_keyword = AstNode::from_pest(&mut context)?;

        println!("VariableDeclaration: {:?}", context.as_str());

        let identifier = AstNode::from_pest(&mut context)?;
        let equal = AstNode::from_pest(&mut context).ok();
        let expression = AstNode::from_pest(&mut context).ok();
        let semi = AstNode::from_pest(&mut context).ok();

        Ok(VariableDeclaration {
            var_decl_keyword,
            identifier,
            equal,
            expression,
            semi,
        })
    }
}

impl PrintAst for VariableDeclaration {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        result.push_str(&self.var_decl_keyword.print_ast(print_properties));
        result.push_str(&self.identifier.print_ast(print_properties));
        if let Some(equal) = &self.equal {
            result.push_str(&equal.print_ast(print_properties));
        }
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
pub enum VarDeclKeyword {
    VarDec(AstNode<AstNode<VarDecKeyword>>),
    Global(AstNode<AstNode<GlobalKeyword>>, Option<AstNode<TypeAnnotation>>),
}

impl FromPest<'_> for VarDeclKeyword {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<'_, Self::Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        let mut current_rule = pest.peek().ok_or(ConversionError::NoMatch)?;
        if current_rule.as_rule() != Rule::VarDeclKeyword {
            return Err(ConversionError::NoMatch);
        }
        current_rule = pest.next().ok_or(ConversionError::NoMatch)?;

        let mut context = &mut current_rule.clone().into_inner();

        let next = find_next_non_comment_or_whitespace(&mut context)?.unwrap().as_rule();
        let var_decl_keyword = match next {
            Rule::VarDec => {
                VarDeclKeyword::VarDec(AstNode::from_pest(&mut context)?)
            },
            Rule::Global => {
                let global_keyword = AstNode::from_pest(&mut context)?;
                let type_annotation = AstNode::from_pest(&mut context).ok();
                VarDeclKeyword::Global(global_keyword, type_annotation)
            }
            _ => return Err(ConversionError::NoMatch),
        };

        Ok(var_decl_keyword)
    }
}

impl PrintAst for VarDeclKeyword {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        match self {
            VarDeclKeyword::VarDec(var_keyword) => var_keyword.print_ast(print_properties),
            VarDeclKeyword::Global(global_keyword, type_annotation) => {
                let mut result = String::new();
                result.push_str(&global_keyword.print_ast(print_properties));
                if let Some(type_annotation) = type_annotation {
                    result.push_str(&type_annotation.print_ast(print_properties));
                }
                result
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::VarDec))]
pub enum VarDecKeyword {
    Var(AstNode<VarKeyword>),
    Type(AstNode<TypeAnnotation>),
}

impl PrintAst for VarDecKeyword {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        match self {
            VarDecKeyword::Var(var_keyword) => var_keyword.print_ast(print_properties),
            VarDecKeyword::Type(type_annotation) => type_annotation.print_ast(print_properties),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::Var))]
pub struct VarKeyword;

impl PrintAst for VarKeyword {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("var")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::Global))]
pub struct GlobalKeyword;

impl PrintAst for GlobalKeyword {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("global")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::Eq))]
pub struct Equal;

impl PrintAst for Equal {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("=")
    }
}