pub mod Lsv4Root;

use from_pest::{ConversionError, FromPest, Void};
use pest::iterators::Pairs;
use crate::lsv4::Rule;
use crate::utils;
use crate::utils::PrintAst;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstNode<T: for<'a> FromPest<'a> + PrintAst> {
    pub data: Box<T>,
    pub meta: AstNodeMeta,
}

impl<'a, T: for<'b> from_pest::FromPest<'b, Rule = Rule, FatalError = Void> + PrintAst> FromPest<'a> for AstNode<T> {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<Self::Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        let mut prev_ignored = Vec::new();
        let mut data = None;
        for pair in pest {
            match pair.as_rule() {
                Rule::COMMENT => {
                    prev_ignored.push(CommentOrWhitespace::Comment(pair.as_str().to_string()));
                }
                Rule::WHITESPACE => {
                    prev_ignored.push(CommentOrWhitespace::Whitespace(pair.as_str().to_string()));
                }
                _ => {
                    data = Some(T::from_pest(&mut Pairs::single(pair))?);
                    break;
                }
            }
        }
        Ok(AstNode {
            data: Box::new(data.ok_or(ConversionError::NoMatch)?),
            meta: AstNodeMeta { prev_ignored },
        })
    }
}

impl<T: for<'a> FromPest<'a> + PrintAst> PrintAst for AstNode<T> {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        for comment_or_whitespace in &self.meta.prev_ignored {
            result.push_str(&comment_or_whitespace.print_ast(print_properties));
        }
        result.push_str(&self.data.print_ast(print_properties));
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommentOrWhitespace {
    Comment(String),
    Whitespace(String),
}

impl PrintAst for CommentOrWhitespace {
    fn print_ast(&self, print_properties: utils::PrintProperties) -> String {
        match self {
            CommentOrWhitespace::Comment(comment) => {
                if print_properties.keep_comments {
                    comment.clone()
                } else {
                    String::new()
                }
            }
            CommentOrWhitespace::Whitespace(whitespace) => {
                if print_properties.simplify_whitespace {
                    // Define the indentation level
                    const INDENTATION: &str = "    ";

                    // Replace tabs with spaces and remove redundant spaces
                    let simplified_whitespace = regex::Regex::new(r" {2,}")
                        .unwrap()
                        .replace_all(&whitespace.replace("\t", INDENTATION), " ")
                        .to_string();

                    // Apply indentation to each line, including empty lines
                    simplified_whitespace.split('\n')
                        .filter(|line| !line.is_empty())
                        .map(|line| format!("{}{}", INDENTATION, line))
                        .collect::<Vec<String>>()
                        .join("\n")
                } else {
                    whitespace.clone()
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstNodeMeta {
    pub prev_ignored: Vec<CommentOrWhitespace>,
}