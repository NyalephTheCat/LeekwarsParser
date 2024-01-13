use pest_ast::FromPest;
use crate::ast::AstNode;
use crate::ast::lsv4root::Statement;
use crate::utils::PrintAst;
use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::BlockStatement))]
pub struct BlockStatement {
    pub lbrace: AstNode<Lbrace>,
    pub statements: Vec<AstNode<Statement>>,
    pub rbrace: AstNode<Rbrace>,
}

impl PrintAst for BlockStatement {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        result.push_str(&self.lbrace.print_ast(print_properties));
        for statement in &self.statements {
            result.push_str(&statement.print_ast(print_properties));
        }
        result.push_str(&self.rbrace.print_ast(print_properties));
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::LBrace))]
pub struct Lbrace;

impl PrintAst for Lbrace {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("{")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(FromPest)]
#[pest_ast(rule(Rule::RBrace))]
pub struct Rbrace;

impl PrintAst for Rbrace {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("}")
    }
}