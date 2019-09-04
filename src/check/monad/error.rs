use std::fmt::{Display, Error as FmtError, Formatter};

use crate::syntax::abs::Abs;
use crate::syntax::common::{Ident, SyntaxInfo, VarRec, DBI, MI};
use crate::syntax::core::{TVal, Val};
use voile_util::level::Level;

/// Type-Checking Error.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TCE {
    Textual(String),

    // == "Cannot"s ==
    CannotInfer(SyntaxInfo, Abs),
    CannotUnify(Val, Val),

    // == "Not"s ==
    NotSigma(SyntaxInfo, TVal),
    NotPi(SyntaxInfo, TVal),
    /// Expected the first `TVal` to be the subtype of
    /// the second `TVal`.
    NotSubtype(TVal, TVal),
    NotTypeAbs(SyntaxInfo, Abs),
    NotTypeVal(SyntaxInfo, Val),
    NotRowType(VarRec, SyntaxInfo, Val),
    NotRecVal(SyntaxInfo, Val),
    NotUniverseVal(SyntaxInfo, Val),

    // == Elaboration ==
    TypeNotInGamma(SyntaxInfo),
    OverlappingVariant(SyntaxInfo, String),
    DuplicateField(SyntaxInfo, String),
    UnexpectedVariant(SyntaxInfo, String),
    MissingVariant(VarRec, String),
    /// Maximum `DBI` vs. Requested `DBI`
    DbiOverflow(DBI, DBI),
    /// Expected the first level to be smaller than second.
    /// The `String` represents the expression.
    LevelMismatch(SyntaxInfo, Level, Level),
    /// Cannot find the definition.
    LookUpFailed(Ident),
    Wrapped(Box<Self>, SyntaxInfo),

    // == Scoping ==
    /// The definition at the first `SyntaxInfo` will
    /// hide the definition at the second `SyntaxInfo`.
    ReDefine(SyntaxInfo, SyntaxInfo),

    // == "Meta"s ==
    /// Recursive metas are disallowed.
    MetaRecursion(MI),
    /// Meta solution should be passed with bound variables only.
    MetaWithNonVar(SyntaxInfo),
    /// Unsolved metas are reported as errors.
    MetaUnsolved(MI),
}

impl TCE {
    pub fn wrap(self, info: SyntaxInfo) -> Self {
        TCE::Wrapped(Box::new(self), info)
    }

    pub fn duplicate_field(ident: Ident) -> Self {
        TCE::DuplicateField(ident.info, ident.text)
    }
}

impl Display for TCE {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            TCE::Textual(text) => f.write_str(text),
            TCE::CannotInfer(id, val) => write!(f, "Could not infer type of: `{}` at {}.", val, id),
            TCE::TypeNotInGamma(id) => write!(f, "Type info not in Gamma for: {}.", id),
            TCE::NotSigma(id, val) => write!(
                f,
                "Expected a sigma type expression, got: `{}` at {}.",
                val, id
            ),
            TCE::NotPi(id, val) => write!(
                f,
                "Expected a pi type expression (function), got: `{}` at {}.",
                val, id
            ),
            TCE::CannotUnify(val1, val2) => write!(f, "Cannot unify `{}` with `{}`.", val1, val2),
            TCE::NotSubtype(sub, sup) => {
                write!(f, "Expected `{}` to be the subtype of `{}`.", sub, sup)
            }
            TCE::NotTypeAbs(id, abs) => {
                write!(f, "Expected a type expression, got: `{}` at {}.", abs, id)
            }
            TCE::NotTypeVal(id, val) => {
                write!(f, "Expected a type expression, got: `{}` at {}.", val, id)
            }
            TCE::NotRecVal(id, val) => {
                write!(f, "Expected a record expression, got: `{}` at {}.", val, id)
            }
            TCE::NotRowType(VarRec::Record, id, val) => write!(
                f,
                "Expected a record type expression, got: `{}` at {}.",
                val, id
            ),
            TCE::NotRowType(VarRec::Variant, id, val) => write!(
                f,
                "Expected a variant type expression, got: `{}` at {}.",
                val, id
            ),
            TCE::MissingVariant(VarRec::Variant, variant) => {
                write!(f, "Missing variant `{}`.", variant)
            }
            TCE::MissingVariant(VarRec::Record, field) => write!(f, "Missing field `{}`.", field),
            TCE::OverlappingVariant(id, variant) => {
                write!(f, "Duplicated variant: `{}` at {}.", variant, id)
            }
            TCE::DuplicateField(id, variant) => {
                write!(f, "Duplicated field: `{}` at {}.", variant, id)
            }
            TCE::UnexpectedVariant(id, variant) => {
                write!(f, "Unexpected variant: `{}` at {}.", variant, id)
            }
            TCE::NotUniverseVal(id, val) => write!(
                f,
                "Expected an universe expression, got: `{}` at {}.",
                val, id
            ),
            TCE::DbiOverflow(expected, actual) => write!(
                f,
                "DBI overflow, maximum: `{}`, got: `{}`.",
                expected, actual
            ),
            TCE::LookUpFailed(var) => {
                write!(f, "Look up failed for `{}` at {}", var.text, var.info)
            }
            TCE::LevelMismatch(expr, expected_to_be_small, big) => write!(
                f,
                "Expression `{}` has level {}, which is not smaller than {}.",
                expr, expected_to_be_small, big
            ),
            TCE::Wrapped(inner, info) => {
                write!(f, "{}\nWhen checking the expression at: {}.", inner, info)
            }
            TCE::ReDefine(new, old) => write!(
                f,
                "The definition at {} will hide the definition at {}.",
                new, old
            ),
            TCE::MetaRecursion(mi) => write!(
                f,
                "Failed to solve meta {:?}: \
                 anticipated solution contains recursive call.",
                mi
            ),
            TCE::MetaUnsolved(mi) => write!(f, "Failed to solve meta {:?}: No solution found.", mi),
            TCE::MetaWithNonVar(info) => write!(
                f,
                "Failed to solve meta at {}: \
                 anticipated solution contains unexpected non-bound values.",
                info
            ),
        }
    }
}
