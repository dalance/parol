use crate::{ParserError, Token};

use id_tree_layout::Visualize;
use std::fmt::{Display, Formatter};

///
/// The type of the elements in the parse tree.
///
/// The lifetime parameter `'t` refers to the lifetime of the scanned text.
///
#[derive(Debug, Clone)]
pub enum ParseTreeType<'t> {
    ///
    /// A scanned token.
    ///
    T(Token<'t>),

    ///
    /// A non-terminal name.
    /// All names are of static lifetime (see NON_TERMINALS slice of non-terminal names).
    ///
    N(&'static str),
}

impl<'t> ParseTreeType<'t> {
    ///
    /// Tries to access the Token of the ParseTreeType.
    /// Can fail if the entry is no terminal (i.e. a non-terminal).
    ///
    pub fn token(&self) -> Result<&Token<'t>, ParserError> {
        match self {
            Self::T(t) => Ok(t),
            _ => Err(ParserError::InternalError(format!("{} is no token!", self))),
        }
    }
}

///
/// Implementation of the Visualize trait to support the visualization of the
/// ParseTreeType in a tree layout.
///
impl Visualize for ParseTreeType<'_> {
    fn visualize(&self) -> std::string::String {
        match self {
            Self::T(t) => format!("{}", t),
            Self::N(n) => n.to_string(),
        }
    }
    fn emphasize(&self) -> bool {
        matches!(self, Self::T(_))
    }
}

impl Display for ParseTreeType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::T(t) => write!(f, "T({})", t),
            Self::N(n) => write!(f, "N({})", n),
        }
    }
}
