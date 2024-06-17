use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use crate::Condition;

use super::utils::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum CharacterCategory {
    Creature,
    Construct,
    Demon,
    Undead,
}

static_gen_fn_all!(CharacterCategory);

impl Default for CharacterCategory {
    fn default() -> Self {
        Self::Creature
    }
}

impl std::fmt::Display for CharacterCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_case(Case::Lower))
    }
}

impl CharacterCategory {
    pub fn description(&self) -> String {
        match self {
            Self::Creature => String::from("A living creature, such as a human or an animal."),
            Self::Construct => format!(
                "An artificial being animated by magic or technology. Doesn't need to eat, drink, \
                 sleep, or breathe. Immune to poison, disease, fear, terror, and corruption. \
                 Doesn't heal naturally, but must be repaired like an item. Can't be ~{}~. Upon \
                 suffering critical damage it is damaged, like an item, and is disabled until \
                 repaired.",
                Condition::Incapacitated,
            ),
            Self::Demon => format!(
                "A dark being of pure chaos manifested in the material world. Doesn't need to \
                 eat, drink, sleep, or breathe. Immune to poison, disease, fear, and terror. \
                 Can't be ~{}~. If defeated, it doesn't die but is simply banished back from \
                 where it came from.",
                Condition::Incapacitated
            ),
            Self::Undead => format!(
                "A being which is neither alive nor dead. Doesn't need to eat, drink, sleep, or \
                 breathe. Immune to poison, disease, fear, and terror. Can't be ~{}~.",
                Condition::Incapacitated
            ),
        }
    }
}
