// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

#![allow(clippy::all)]

use parol_runtime::id_tree::Tree;
use parol_runtime::lexer::{TokenStream, Tokenizer};
use parol_runtime::miette::Result;
use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{
    ParseTreeType, DFATransition, LLKParser, LookaheadDFA, ParseType, Production,{{^auto_generate?}} UserActionsTrait,{{/auto_generate}}
};
use std::cell::RefCell;
use std::path::Path;
{{#auto_generate?}}
use crate::{{module_name}}::{{user_type_name}};
use crate::{{module_name}}_trait::{{user_type_name}}Auto;{{/auto_generate}}

{{{lexer_source}}}

const MAX_K: usize = {{max_k}};

pub const NON_TERMINALS: &[&str; {{non_terminal_count}}] = &[
{{{non_terminals}}}];

{{{dfa_source}}}
{{{productions}}}
static TOKENIZERS: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| vec![
    ("INITIAL", Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap()),
{{{scanner_builds}}}
]);

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
{{^auto_generate?}}    user_actions: &mut dyn UserActionsTrait<'t>,{{/auto_generate}}{{#auto_generate?}}    user_actions: &mut {{user_type_name}}{{{user_type_life_time}}},{{/auto_generate}}
) -> Result<Tree<ParseTreeType<'t>>> where T: AsRef<Path> {
    let mut llk_parser = LLKParser::new(
        {{start_symbol_index}},
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    let token_stream = RefCell::new(
        TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap(),
    );{{#auto_generate?}}
    // Initialize wrapper
    let mut user_actions = {{user_type_name}}Auto::new(user_actions);{{/auto_generate}}
    let result = llk_parser.parse(token_stream, {{#auto_generate?}}&mut {{/auto_generate}}user_actions);
    match result {
        Ok(()) => Ok(llk_parser.parse_tree),
        Err(e) => Err(e),
    }
}
