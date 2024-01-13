pub mod ast;
pub mod utils;
mod test;

use from_pest::FromPest;
use pest::iterators::Pairs;
use pest::Parser;
use crate::lsv4::Rule;
use crate::utils::PrintAst;
use pest::iterators::Pair;

pub mod lsv4 {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "grammar/lsv4.pest"]
    pub struct Lsv4Parser; // Parser for Leekscript V4
}

fn display_pairs(pairs: Pairs<Rule>, level: usize, exclude_comments_and_whitespace: bool) -> String {
    let mut output = String::new();

    for pair in pairs {
        let rule = pair.as_rule();

        fn concatenate_rules(pair: Pair<Rule>, output: &mut String, level: usize, exclude_comments_and_whitespace: bool) {
            let inner_pairs: Vec<_> = pair.clone().into_inner().collect();

            if inner_pairs.len() == 1 {
                let inner_pair = &inner_pairs[0];
                output.push_str(&format!(" > {:?}", inner_pair.as_rule()));
                concatenate_rules(inner_pair.clone(), output, level, exclude_comments_and_whitespace);
            } else {
                let pair_str = pair.as_str().replace("\n", "\\n").replace("\t", "\\t");
                output.push_str(": ");
                output.push_str(&format!("\"{}\"\n", pair_str));
                for inner_pair in inner_pairs {
                    output.push_str(&"  ".repeat(level + 1));
                    output.push_str("- ");
                    output.push_str(&format!("{:?}", inner_pair.as_rule()));
                    concatenate_rules(inner_pair, output, level + 1, exclude_comments_and_whitespace);
                }
            }
        }

        output.push_str(&"  ".repeat(level));
        output.push_str("- ");
        output.push_str(&format!("{:?}", rule));
        concatenate_rules(pair, &mut output, level, exclude_comments_and_whitespace);
    }

    output
}


fn main() {
    let input = r#"
do a while (b);
    "#;
    let mut pairs = lsv4::Lsv4Parser::parse(Rule::lsv4_root, input).unwrap_or_else(|e| panic!("{}", e));

    println!("{}", display_pairs(pairs.clone(), 0, true));

    let root = ast::lsv4root::Lsv4Root::from_pest(&mut pairs)
        .unwrap_or_else(|e| panic!("{}", e));
    println!("{:#?}", root);

    println!("Reconstructed file:");
    println!("{:}", root.print_ast(utils::PrintProperties {
        keep_comments: true,
        simplify_whitespace: false,
        compact: false,
        indent: 0,
    }));
}
