use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::utils::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum IntelligenceCategory {
    HumanIntelligence,
    AnimalIntelligence,
    Mindless,
}

static_gen_fn_all!(IntelligenceCategory);

impl Default for IntelligenceCategory {
    fn default() -> Self {
        Self::HumanIntelligence
    }
}

impl std::fmt::Display for IntelligenceCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_case(Case::Lower))
    }
}
