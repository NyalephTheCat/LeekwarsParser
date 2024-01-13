use from_pest::FromPest;
use pest::Parser;
use crate::ast::lsv4root::Lsv4Root;
use crate::lsv4;
use crate::utils::{PrintAst, PrintProperties};

/// Parses a string and checks if it returns the same string when displayed
fn symetric_parse(input: &str) {
    let mut pairs = lsv4::Lsv4Parser::parse(lsv4::Rule::lsv4_root, input).unwrap_or_else(|e| panic!("{}", e));
    let ast = Lsv4Root::from_pest(&mut pairs).unwrap_or_else(|e| panic!("{}", e));
    let output = ast.print_ast(PrintProperties::default());
    assert_eq!(input, output);
}

#[test]
fn empty() {
    symetric_parse("");
    symetric_parse("      ");
}

#[test]
fn comments() {
    symetric_parse("/* Some comments */");
    symetric_parse("//An empty statement");
    symetric_parse("/* Some comments */ //An empty statement");
    symetric_parse("//An empty statement\n");
    symetric_parse("/* Some comments \nAnother line on the same comment*/");
}

#[test]
fn empty_statement() {
    symetric_parse(";");
    symetric_parse(";;");
    symetric_parse(";//comment\n;");
    symetric_parse("/*comment*/;");
}

#[test]
fn block_statement() {
    symetric_parse("{}");
    symetric_parse("{;}");
    symetric_parse("{\n;\n}");
}