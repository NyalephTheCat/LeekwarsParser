use from_pest::{ConversionError, Void};
use pest::iterators::{Pair, Pairs};
use crate::lsv4::Rule;

#[derive(Clone, Copy)]
pub struct PrintProperties {
    pub keep_comments: bool,
    pub simplify_whitespace: bool,
    pub compact: bool,
    pub indent: usize,
}

impl Default for PrintProperties {
    fn default() -> Self {
        PrintProperties {
            keep_comments: true,
            simplify_whitespace: false,
            compact: false,
            indent: 0,
        }
    }
}

pub trait PrintAst {
    fn print_ast(&self, print_properties: PrintProperties) -> String;
}

pub fn find_next_non_comment_or_whitespace<'a>(
    pairs: &mut Pairs<'a, Rule>,
) -> Result<Option<Pair<'a, Rule>>, ConversionError<Void>> {
    let mut p = pairs.clone(); // To avoid eating the items
    loop {
        let pair = p.next();
        if let Some(pair) = pair {
            match pair.as_rule() {
                Rule::COMMENT | Rule::WHITESPACE => continue,
                _ => return Ok(Some(pair)),
            }
        } else {
            return Ok(None);
        }
    }
}