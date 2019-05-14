use crate::syntax::abs::Abs;
use crate::syntax::common::{Level, SyntaxInfo, DBI};
use crate::syntax::core::Term;
use crate::syntax::surf::Ident;
use std::fmt::{Display, Error as FmtError, Formatter};

/// Type-Checking Error.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TCE {
    Textual(String),
    CouldNotInfer(SyntaxInfo),
    TypeNotInGamma(SyntaxInfo),
    NotSigma(SyntaxInfo, Term),
    NotPi(SyntaxInfo, Term),
    NotType(SyntaxInfo, Abs),
    /// Maximum `DBI` vs. Requested `DBI`
    DbiOverflow(DBI, DBI),
    /// Expected the first level to be smaller than second.
    /// The `String` represents the expression.
    LevelMismatch(SyntaxInfo, Level, Level),
    /// Cannot find the definition.
    LookUpFailed(Ident),
}

impl Display for TCE {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            TCE::Textual(text) => f.write_str(text),
            TCE::CouldNotInfer(term) => write!(f, "Could not infer type of: {}.", term),
            TCE::TypeNotInGamma(id) => write!(f, "Type info not in Gamma for: {}.", id),
            TCE::NotSigma(id, term) => write!(
                f,
                "Expected a sigma type expression, got: `{:?}` at {}.",
                term, id
            ),
            TCE::NotPi(id, term) => write!(
                f,
                "Expected a pi type expression (function), got: `{:?}` at {}.",
                term, id
            ),
            TCE::NotType(id, abs) => {
                write!(f, "Expected a type expression, got: `{:?}` at {}.", abs, id)
            }
            TCE::DbiOverflow(expected, actual) => write!(
                f,
                "DBI overflow, maximum: `{}`, got: `{}`.",
                expected, actual
            ),
            TCE::LookUpFailed(var) => {
                f.write_str("Look up failed for `")?;
                // more information here?
                var.info.text.fmt(f)?;
                f.write_str("`, ")
            }
            TCE::LevelMismatch(expr, expected_to_be_small, big) => write!(
                f,
                "Expression `{}` has level {:?}, which is not smaller than {:?}.",
                expr, expected_to_be_small, big
            ),
        }
    }
}
