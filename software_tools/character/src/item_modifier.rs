use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum CostModifier {
    Multiply(i32),
    Divide(i32),
}

impl std::fmt::Display for CostModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Multiply(x) => write!(f, "×{x}"),
            Self::Divide(x) => write!(f, "÷{x}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum ItemModifier {
    FragileItem,
    RobustItem,
    MastercraftedWeapon,
    MastercraftedArmour,
    AnimalArmour,
    SturdyAnimalArmour,
    ExpensiveItem,
    LuxuryItem,
}

static_gen_fn_all!(ItemModifier);

impl std::fmt::Display for ItemModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_case(Case::Lower))
    }
}

impl ItemModifier {
    pub fn cost_modifier(&self) -> CostModifier {
        match self {
            Self::FragileItem => CostModifier::Divide(4),
            Self::RobustItem => CostModifier::Multiply(4),
            Self::MastercraftedWeapon => CostModifier::Multiply(4),
            Self::MastercraftedArmour => CostModifier::Multiply(4),
            Self::AnimalArmour => CostModifier::Multiply(1),
            Self::SturdyAnimalArmour => CostModifier::Multiply(4),
            Self::ExpensiveItem => CostModifier::Multiply(4),
            Self::LuxuryItem => CostModifier::Multiply(16),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::FragileItem => format!("Gains the ~{}~ keyword.", ItemKeyword::Fragile),
            Self::RobustItem => format!("Gains the ~{}~ keyword.", ItemKeyword::Robust),
            Self::MastercraftedWeapon => {
                format!("The damage die is improved: d4 to d6, d6 to d8, d8 to d10, d10 to d12.",)
            }
            Self::MastercraftedArmour => format!("Bulk is halved."),

            Self::AnimalArmour => format!(
                "Made for dogs or other medium-sized animals without the ~{}~ trait.",
                Trait::Sturdy
            ),
            Self::SturdyAnimalArmour => format!(
                "Bulk is doubled. Made for horses or other medium-sized animals with the ~{}~ \
                 trait.",
                Trait::Sturdy,
            ),
            Self::ExpensiveItem => format!(
                "A particularly valuable version of another item. Good wine is an expensive ~{}~, \
                 a decorated sword is an expensive ~{}~, fancy outfits are expensive ~{}~, etc.",
                ItemKind::AlcoholicDrink,
                ItemKind::MartialHandWeapon,
                ItemKind::Clothes
            ),
            Self::LuxuryItem => format!(
                "An extremely valuable version of another item. Fine spirit is a luxury ~{}~, a \
                 gem-encrusted sword is a luxury ~{}~, aristocratic outfits are luxury ~{}~, etc.",
                ItemKind::AlcoholicDrink,
                ItemKind::MartialHandWeapon,
                ItemKind::Clothes
            ),
        }
    }
}
