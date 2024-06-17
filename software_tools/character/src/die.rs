use convert_case::{Case, Casing};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
}

impl std::fmt::Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{}", format!("{self:?}").to_case(Case::Flat)),
        }
    }
}
