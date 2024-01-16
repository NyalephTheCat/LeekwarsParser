use from_pest::{ConversionError, FromPest, Void};
use pest::iterators::Pairs;
use crate::lsv4::Rule;
use crate::utils::PrintAst;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeAnnotation;

impl PrintAst for TypeAnnotation {
    fn print_ast(&self, _print_properties: crate::utils::PrintProperties) -> String {
        String::from("TYPE ANNOTATION")
    }
}

impl FromPest<'_> for TypeAnnotation {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<'_, Self::Rule>) -> Result<Self, ConversionError<Self::FatalError>> {
        if pest.peek().is_none() {
            return Err(ConversionError::Extraneous { current_node: "Type annotation" })
        }
        if pest.peek().unwrap().as_rule() != Rule::Type {
            return Err(ConversionError::NoMatch);
        }

        // TODO - actually parse the type
        pest.next();

        Ok(TypeAnnotation)
    }
}