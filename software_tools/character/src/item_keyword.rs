use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum ItemKeyword {
    Armour(i8),
    Consumable,
    Durability(i8, i8),
    Fragile,
    Robust,
    Shield,
    Vehicle(i8),
    Weapon(Weapon),
}

static_gen_fn_all!(ItemKeyword);

impl ItemKeyword {
    pub fn description(&self) -> String {
        match self {
            Self::Armour(_) => {
                format!(
                    "Increases armour value by X while worn. Characters can only wear one suit of \
                     armour. Equipping or unequipping armour takes a stretch. Armour is fitted to \
                     a specific character, and only those of similar size are able to wear it. \
                     When in doubt, there is a 1:8 chance it is possible."
                )
            }
            Self::Shield => String::from(
                "Increases the armour value by 1 while held but only against attacks you dodge, \
                 attacks you counter, and attacks used to counter you. If you hold two shields at \
                 once you still only increase armour value by 1, but against all attacks. Doesn't \
                 require two hands to use despite its bulk.",
            ),
            Self::Consumable => format!("Discarded after use."),
            Self::Durability(_, _) => {
                format!(
                    "This item has health equal to X and armour value equal to Y, and suffers \
                     damage like characters instead of using durability rolls. On critical \
                     damage, it is ~damaged~ and can't be used until it recovers at least 1 \
                     health. Items can't heal like characters, but can be repaired. Sources of \
                     damage which can't realistically damage the items are completely ineffective \
                     (such as a sword against a metal door)."
                )
            }
            Self::Fragile => format!(
                "Behaves as an already ~damaged~ item: it is destroyed on a failed durability \
                 roll, weapons are destroyed on a roll of 1, shields and armour are destroyed \
                 upon taking 8 or more damage."
            ),
            Self::Robust => format!("Only has a 1:8 chance of failing durability rolls."),
            Self::Weapon(_) => {
                format!(
                    "Can be used to perform an attack while held, with the indicated damage and \
                     properties. Attacks made with one-handed weapons held in the non-dominant \
                     hand inflict impaired damage. Weapons of bulk 2 or more require both hands \
                     to use and can't be used to attack characters you are grabbing. You don't \
                     need to track ammunition for ranged weapons, it is assumed you are carrying \
                     enough and that durability rolls also account for it."
                )
            }
            Self::Vehicle(_) => {
                format!(
                    "Can carry X {} characters and their gear. A character can be replaced by \
                     items with a total bulk of 16. The bulk of what's on the vehicle doesn't \
                     count towards the encumbrance of characters or operating it.",
                    SizeCategory::MediumSized
                )
            }
        }
    }
}

impl std::fmt::Display for ItemKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Armour(x) => write!(f, "armour {x}"),
            Self::Durability(x, y) => write!(f, "durability {x}/{y}"),
            Self::Weapon(x) => write!(f, "weapon ({x})"),
            Self::Vehicle(x) => write!(f, "vehicle {x}"),
            _ => write!(f, "{}", format!("{self:?}").to_case(Case::Lower)),
        }
    }
}
