pub mod lsv4root;
pub mod eoi;
pub mod semi;
pub mod block_statement;
pub mod statement;
mod return_statement;
mod expression;
mod break_statement;
mod continue_statement;
mod while_statement;
mod do_while_statement;
mod if_statement;

use from_pest::{ConversionError, FromPest, Void};
use pest::iterators::Pairs;
use crate::lsv4::Rule;
use crate::utils;
use crate::utils::PrintAst;
use std::fmt::Pointer;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AstNode<T: for<'a> FromPest<'a> + PrintAst> {
    pub data: Box<T>,
    pub meta: AstNodeMeta,
}

impl<T: for<'a> FromPest<'a> + PrintAst + std::fmt::Debug> std::fmt::Debug for AstNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            return f.debug_struct("AstNode")
                .field("data", &self.data)
                .field("nb_children", &self.meta.prev_ignored.len())
                .finish();
        }

        f.debug_struct("AstNode")
            .field("data", &self.data)
            .field("meta", &self.meta)
            .finish()
    }
}

impl<'a, T: for<'b> FromPest<'b, Rule = Rule, FatalError = Void> + PrintAst> FromPest<'a> for AstNode<T> {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<'a, Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        let mut meta = AstNodeMeta {
            prev_ignored: Vec::new(),
            post_ignored: Vec::new(),
        };

        let mut context = pest.clone();

        // Extract preceding comments and whitespace
        meta.prev_ignored = extract_comments_or_whitespace(&mut context, true)?;

        let data = Box::new(T::from_pest(&mut context)?);

        // Check if the next significant node is not a comment or whitespace
        if is_last_significant_node(&mut context) {
            meta.post_ignored = extract_comments_or_whitespace(&mut context, false)?;
        }

        *pest = context;

        Ok(AstNode { data, meta })
    }
}

// Extracts comments or whitespace based on the direction (before or after the node)
fn extract_comments_or_whitespace(pest: &mut Pairs<Rule>, before_node: bool) -> Result<Vec<CommentOrWhitespace>, ConversionError<Void>> {
    let mut items = Vec::new();

    while let Some(_) = pest.peek() {
        if let Ok(comment_or_whitespace) = CommentOrWhitespace::from_pest(pest) {
            items.push(comment_or_whitespace);
        } else {
            break;
        }
    }

    Ok(items)
}

// Checks if the current position in the pest iterator is at the last significant node
fn is_last_significant_node(pest: &mut Pairs<Rule>) -> bool {
    let remaining = pest.clone().filter(|p| p.as_rule() != Rule::COMMENT && p.as_rule() != Rule::WHITESPACE);
    remaining.count() == 0
}

impl<T: for<'a> FromPest<'a> + PrintAst> PrintAst for AstNode<T> {
    fn print_ast(&self, print_properties: crate::utils::PrintProperties) -> String {
        let mut result = String::new();
        for comment_or_whitespace in &self.meta.prev_ignored {
            result.push_str(&comment_or_whitespace.print_ast(print_properties));
        }
        result.push_str(&self.data.print_ast(print_properties));
        for comment_or_whitespace in &self.meta.post_ignored {
            result.push_str(&comment_or_whitespace.print_ast(print_properties));
        }
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
    pub post_ignored: Vec<CommentOrWhitespace>, // Ignored after the node if it's the last node
}