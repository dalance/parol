use crate::list_grammar_trait::ListGrammarTrait;
use anyhow::Context;
use id_tree::Tree;
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType};
use parol_runtime::Result;
use std::fmt::{Debug, Display, Error, Formatter};

///
/// The value range for the supported list elements
///
pub type DefinitionRange = usize;

///
/// Data structure that implements the semantic actions for our list grammar
///
#[derive(Debug, Default)]
pub struct ListGrammar {
    pub numbers: Vec<DefinitionRange>,
}

impl ListGrammar {
    pub fn new() -> Self {
        ListGrammar::default()
    }

    fn push(&mut self, item: DefinitionRange) {
        self.numbers.push(item)
    }
}

impl Display for ListGrammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        writeln!(
            f,
            "[{}]",
            self.numbers
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl ListGrammarTrait for ListGrammar {
    /// Semantic action for production 6:
    ///
    /// Num: "[0-9]+";
    ///
    fn num(&mut self, num: &ParseTreeStackEntry, parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        let symbol = num.text(parse_tree)?;
        let number = symbol
            .parse::<DefinitionRange>()
            .context("num_6: Error accessing token from ParseTreeStackEntry")?;
        self.push(number);
        Ok(())
    }
}
