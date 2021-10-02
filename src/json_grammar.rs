use crate::json_grammar_trait::JsonGrammarTrait;
use id_tree::Tree;
use log::trace;
use parol_runtime::parser::errors::*;
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType};
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure used to build up a json structure item during parsing
///
#[derive(Debug, Clone)]
pub enum JsonGrammarItem {
    Null,
    True,
    False,
    String(String),
    Number(f64),
    Array(Vec<JsonGrammarItem>),
    Pair((String, Box<JsonGrammarItem>)),
    Object(Vec<JsonGrammarItem>),
}

impl Display for JsonGrammarItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match self {
            Self::Null => write!(f, "Null"),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::String(s) => write!(f, "{}", s),
            Self::Number(n) => write!(f, "{}", n),
            Self::Array(v) => write!(
                f,
                "[{}]",
                v.iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Pair((s, v)) => write!(f, "{}: {}", s, v),
            Self::Object(p) => write!(
                f,
                "{{{}}}",
                p.iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

///
/// Data structure used to build up a json structure during parsing
///
#[derive(Debug, Default)]
pub struct JsonGrammar {
    pub item_stack: Vec<JsonGrammarItem>,
}

impl JsonGrammar {
    pub fn new() -> Self {
        JsonGrammar::default()
    }

    fn push(&mut self, item: JsonGrammarItem, context: &str) {
        trace!("push   {}: {}", context, item);
        self.item_stack.push(item)
    }

    fn pop(&mut self, context: &str) -> Option<JsonGrammarItem> {
        if !self.item_stack.is_empty() {
            let item = self.item_stack.pop();
            if let Some(ref item) = item {
                trace!("pop    {}: {}", context, item);
            }
            item
        } else {
            None
        }
    }

    #[allow(dead_code)]
    // Use this function for debugging purposes:
    // $env:RUST_LOG="json_parser::json_grammar=trace"
    // trace!("{}", self.trace_ast_stack(context));
    fn trace_ast_stack(&self, context: &str) -> String {
        format!(
            "Ast stack at {}:\n{}",
            context,
            self.item_stack
                .iter()
                .rev()
                .map(|s| format!("  {}", s))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Display for JsonGrammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        writeln!(
            f,
            "{}",
            self.item_stack
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl JsonGrammarTrait for JsonGrammar {
    /// Semantic action for production 1:
    ///
    /// Object: "\{" ObjectSuffix1;
    ///
    fn object_1(
        &mut self,
        _l_brace_0: &ParseTreeStackEntry,
        _object_suffix1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "object_1";
        let top_of_stack = self.pop(context);
        match &top_of_stack {
            Some(JsonGrammarItem::Object(pairs)) => {
                let mut pairs = pairs.clone();
                pairs.reverse();
                self.push(JsonGrammarItem::Object(pairs.to_vec()), context);
                Ok(())
            }
            _ => Err(format!("{}: unexpected ({:?}", context, top_of_stack).into()),
        }
    }

    /// Semantic action for production 2:
    ///
    /// ObjectSuffix1: Pair ObjectSuffix;
    ///
    fn object_suffix1_2(
        &mut self,
        _pair_0: &ParseTreeStackEntry,
        _object_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "object_suffix1_2";
        let top_of_stack1 = self.pop(context);
        let top_of_stack2 = self.pop(context);
        match (&top_of_stack1, &top_of_stack2) {
            (Some(JsonGrammarItem::Object(pairs)), Some(pair)) => {
                let mut pairs = pairs.clone();
                pairs.push(pair.clone());
                self.push(JsonGrammarItem::Object(pairs.to_vec()), context);
                Ok(())
            }
            _ => Err(format!(
                "{}: unexpected ({:?}, {:?}",
                context, top_of_stack1, top_of_stack2
            )
            .into()),
        }
    }

    /// Semantic action for production 5:
    ///
    /// ObjectSuffix: "\}";
    ///
    fn object_suffix_5(
        &mut self,
        _r_brace_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "object_suffix_5";
        self.push(JsonGrammarItem::Object(Vec::new()), context);
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// ObjectRest: "," Pair ObjectRestSuffix;
    ///
    fn object_rest_6(
        &mut self,
        _comma_0: &ParseTreeStackEntry,
        _pair_1: &ParseTreeStackEntry,
        _object_rest_suffix_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "object_rest_6";
        let top_of_stack1 = self.pop(context);
        let top_of_stack2 = self.pop(context);
        match (&top_of_stack1, &top_of_stack2) {
            (Some(JsonGrammarItem::Object(pairs)), Some(pair)) => {
                let mut pairs = pairs.clone();
                pairs.push(pair.clone());
                self.push(JsonGrammarItem::Object(pairs.to_vec()), context);
                Ok(())
            }
            _ => Err(format!(
                "{}: unexpected ({:?}, {:?}",
                context, top_of_stack1, top_of_stack2
            )
            .into()),
        }
    }

    /// Semantic action for production 8:
    ///
    /// ObjectRestSuffix: ;
    ///
    fn object_rest_suffix_8(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        let context = "object_rest_suffix_8";
        self.push(JsonGrammarItem::Object(Vec::new()), context);
        Ok(())
    }

    /// Semantic action for production 9:
    ///
    /// Pair: String ":" Value;
    ///
    fn pair_9(
        &mut self,
        _string_0: &ParseTreeStackEntry,
        _colon_1: &ParseTreeStackEntry,
        _value_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "pair_9";
        let value = self.pop(context);
        let name = self.pop(context);
        match (&name, &value) {
            (Some(JsonGrammarItem::String(string)), Some(value)) => {
                self.push(
                    JsonGrammarItem::Pair((string.to_string(), Box::new(value.clone()))),
                    context,
                );
                Ok(())
            }
            _ => Err(format!("{}: unexpected ({:?}, {:?}", context, value, name).into()),
        }
    }

    /// Semantic action for production 10:
    ///
    /// Array: "\[" ArraySuffix1;
    ///
    fn array_10(
        &mut self,
        _l_bracket_0: &ParseTreeStackEntry,
        _array_suffix1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "array_10";
        let top_of_stack = self.pop(context);
        match &top_of_stack {
            Some(JsonGrammarItem::Array(list)) => {
                let mut list = list.clone();
                list.reverse();
                self.push(JsonGrammarItem::Array(list.to_vec()), context);
                Ok(())
            }
            _ => Err(format!("{}: unexpected ({:?}", context, top_of_stack).into()),
        }
    }

    /// Semantic action for production 11:
    ///
    /// ArraySuffix1: Value ArraySuffix;
    ///
    fn array_suffix1_11(
        &mut self,
        _value_0: &ParseTreeStackEntry,
        _array_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "array_suffix1_11";
        let top_of_stack1 = self.pop(context);
        let top_of_stack2 = self.pop(context);
        match (&top_of_stack1, &top_of_stack2) {
            (Some(JsonGrammarItem::Array(array)), Some(elem)) => {
                let mut list = array.clone();
                list.push(elem.clone());
                self.push(JsonGrammarItem::Array(list.to_vec()), context);
                Ok(())
            }
            _ => Err(format!(
                "{}: unexpected ({:?}, {:?}",
                context, top_of_stack1, top_of_stack2
            )
            .into()),
        }
    }

    /// Semantic action for production 12:
    ///
    /// ArraySuffix1: "\]";
    ///
    fn array_suffix1_12(
        &mut self,
        _r_bracket_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "array_suffix1_12";
        self.push(JsonGrammarItem::Array(Vec::new()), context);
        Ok(())
    }

    /// Semantic action for production 15:
    ///
    /// ArrayRest: "," Value ArrayRestSuffix;
    ///
    fn array_rest_15(
        &mut self,
        _comma_0: &ParseTreeStackEntry,
        _value_1: &ParseTreeStackEntry,
        _array_rest_suffix_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "array_rest_15";
        let top_of_stack1 = self.pop(context);
        let top_of_stack2 = self.pop(context);
        match (&top_of_stack1, &top_of_stack2) {
            (Some(JsonGrammarItem::Array(array)), Some(elem)) => {
                let mut list = array.clone();
                list.push(elem.clone());
                self.push(JsonGrammarItem::Array(list.to_vec()), context);
                Ok(())
            }
            _ => Err(format!(
                "{}: unexpected ({:?}, {:?}",
                context, top_of_stack1, top_of_stack2
            )
            .into()),
        }
        // Ok(())
    }

    /// Semantic action for production 17:
    ///
    /// ArrayRestSuffix: ;
    ///
    fn array_rest_suffix_17(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        let context = "array_rest_suffix_17";
        self.push(JsonGrammarItem::Array(Vec::new()), context);
        Ok(())
    }

    /// Semantic action for production 22:
    ///
    /// Value: "true";
    ///
    fn value_22(
        &mut self,
        _true_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "value_22";
        self.push(JsonGrammarItem::True, context);
        Ok(())
    }

    /// Semantic action for production 23:
    ///
    /// Value: "false";
    ///
    fn value_23(
        &mut self,
        _false_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "value_23";
        self.push(JsonGrammarItem::False, context);
        Ok(())
    }

    /// Semantic action for production 24:
    ///
    /// Value: "null";
    ///
    fn value_24(
        &mut self,
        _null_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "value_24";
        self.push(JsonGrammarItem::Null, context);
        Ok(())
    }

    /// Semantic action for production 25:
    ///
    /// String: "\u{0022}(\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\\u0000-\u001F])*\u{0022}";
    ///
    fn string_25(
        &mut self,
        string_0: &ParseTreeStackEntry,
        parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "string_25";
        let string = string_0.symbol(parse_tree)?;
        self.push(JsonGrammarItem::String(string.to_string()), context);
        Ok(())
    }

    /// Semantic action for production 26:
    ///
    /// Number: "-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][-+]?(0|[1-9][0-9]*)?)?";
    ///
    fn number_26(
        &mut self,
        number_0: &ParseTreeStackEntry,
        parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "number_26";
        let number = number_0
            .symbol(parse_tree)?
            .parse::<f64>()
            .chain_err(|| format!("{}: Error accessing number token", context))?;
        self.push(JsonGrammarItem::Number(number), context);
        Ok(())
    }
}
