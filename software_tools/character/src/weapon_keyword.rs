use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WeaponKeyword {
    Blast,
    DirectDamage,
    FireDamage,
    Grab,
    HolyDamage,
    IndirectRange(i8),
    Poison(&'static str),
    Range(i8),
    Reload,
    UsageLimit(&'static str),
    Stun,
}

static_gen_fn_all!(WeaponKeyword);

impl std::fmt::Display for WeaponKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IndirectRange(x) => write!(f, "indirect range {x}"),
            Self::Range(x) => write!(f, "range {x}"),
            Self::Poison(x) => write!(f, "{x} poison"),
            Self::UsageLimit(x) => write!(f, "usage limit: {x}"),
            _ => write!(f, "{}", format!("{self:?}").to_case(Case::Lower)),
        }
    }
}

impl WeaponKeyword {
    pub fn description(&self) -> String {
        match self {
            Self::IndirectRange(_) => format!(
                "Can be used to attack up to range X. Can attack over obstacles, but can't attack \
                 within half range.",
            ),
            Self::Range(_) => format!("Can be used to attack up to range X."),
            Self::Blast => {
                format!("Targets all characters in a small zone and on its borders.")
            }
            Self::Grab => format!("Targets must pass a STR save to avoid being grabbed."),
            Self::Poison(_) => format!(
                "The target is exposed to poison of the specified type if they suffer at least 1 \
                 damage.",
            ),
            Self::DirectDamage => {
                format!("Inflicts full damage, ignoring the target's armour value.")
            }
            Self::HolyDamage => {
                format!(
                    "Only effective against ~{}s~ and ~{}~.",
                    CharacterCategory::Demon,
                    CharacterCategory::Undead
                )
            }
            Self::FireDamage => format!("Inflicts fire damage (it is direct damage)."),
            Self::Stun => format!("Targets are ~{}~.", Condition::Stunned),
            Self::Reload => format!(
                "This weapon can only be used only once per combat, since it takes a relative \
                 long time to reload it. In the few situations where it matters, a character can \
                 spend 10 rounds (1 minute) to reload the weapon.",
            ),
            Self::UsageLimit(_) => format!("Can only be used with the specified frequency."),
        }
    }
}
