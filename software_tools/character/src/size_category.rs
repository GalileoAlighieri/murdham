use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::utils::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum SizeCategory {
    Tiny,
    Small,
    MediumSized,
    Large,
    Massive,
}

static_gen_fn_all!(SizeCategory);

impl Default for SizeCategory {
    fn default() -> Self {
        Self::MediumSized
    }
}

impl std::fmt::Display for SizeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MediumSized => write!(f, "medium-sized"),
            _ => write!(f, "{}", format!("{self:?}").to_case(Case::Lower)),
        }
    }
}
