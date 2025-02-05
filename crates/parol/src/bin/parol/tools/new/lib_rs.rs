use derive_builder::Builder;

#[derive(Builder, Debug, Default)]
pub struct LibRsData<'a> {
    crate_name: &'a str,
    grammar_name: String,
    tree_gen: bool,
}

impl std::fmt::Display for LibRsData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let LibRsData {
            crate_name,
            grammar_name,
            tree_gen,
        } = self;

        if *tree_gen {
            f.write_fmt(ume::ume! {
                use parol_runtime::id_tree::Tree;
                use parol_runtime::id_tree_layout::Layouter;
                use parol_runtime::parser::ParseTreeType;
            })?;

            write!(f, "\n\n")?;
        }

        f.write_fmt(ume::ume!(
            extern crate parol_runtime;
        ))?;

        write!(f, "\n\n")?;

        write!(
            f,
            "\
mod {crate_name}_grammar;
pub use {crate_name}_grammar::{grammar_name}Grammar;

mod {crate_name}_grammar_trait;
pub use {crate_name}_grammar_trait::ASTType;

mod {crate_name}_parser;
pub use {crate_name}_parser::parse;
"
        )?;

        if *tree_gen {
            write!(f, "\n\n")?;
            f.write_fmt(ume::ume! {
                pub fn generate_tree_layout(
                    syntax_tree: &Tree<ParseTreeType>,
                    input_file_name: &str,
                ) -> id_tree_layout::layouter::Result {
                    let mut svg_full_file_name = std::path::PathBuf::from(input_file_name);
                    svg_full_file_name.set_extension("svg");

                    Layouter::new(syntax_tree)
                        .with_file_path(&svg_full_file_name)
                        .write()
                }
            })?;
        }

        Ok(())
    }
}
