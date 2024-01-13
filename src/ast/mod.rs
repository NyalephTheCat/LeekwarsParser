pub mod lsv4root;
pub mod eoi;
pub mod semi;
pub mod block_statement;
pub mod statement;
mod return_statement;
mod expression;
mod break_statement;
mod continue_statement;

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

impl<'a, T: for<'b> FromPest<'b, Rule = Rule, FatalError = Void> + PrintAst> FromPest<'a> for AstNode<T> {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<'a, Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        let mut meta = AstNodeMeta {
            prev_ignored: Vec::new(),
        };
        let mut context = pest.clone();
        loop {
            let comment_or_whitespace = CommentOrWhitespace::from_pest(&mut context);
            if let Ok(comment_or_whitespace) = comment_or_whitespace {
                meta.prev_ignored.push(comment_or_whitespace);
            } else {
                break;
            }
        }

        let data = Box::new(T::from_pest(&mut context)?);

        // Set pest to the context after the last parsed rule
        *pest = context;

        Ok(AstNode {
            data,
            meta,
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

impl FromPest<'_> for CommentOrWhitespace {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<Self::Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        let mut current_rule = pest.peek().ok_or(ConversionError::NoMatch)?;
        if current_rule.as_rule() != Rule::COMMENT && current_rule.as_rule() != Rule::WHITESPACE {
            return Err(ConversionError::NoMatch);
        }
        current_rule = pest.next().ok_or(ConversionError::NoMatch)?;

        let mut context = current_rule.clone().into_inner();
        let comment_or_whitespace = match current_rule.as_rule() {
            Rule::COMMENT => {
                let comment = context.next().ok_or(ConversionError::NoMatch)?.as_str().to_string();
                CommentOrWhitespace::Comment(comment)
            }
            Rule::WHITESPACE => {
                let whitespace = context.next().ok_or(ConversionError::NoMatch)?.as_str().to_string();
                CommentOrWhitespace::Whitespace(whitespace)
            }
            _ => return Err(ConversionError::NoMatch),
        };
        Ok(comment_or_whitespace)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstNodeMeta {
    pub prev_ignored: Vec<CommentOrWhitespace>,
}