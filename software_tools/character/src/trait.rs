use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Trait {
    Addicted,
    Fast,
    Frail,
    Frightening,
    HoveringFlyer,
    Immobile,
    Incorporeal,
    LandingFlyer,
    Mutation,
    Regeneration,
    Slow,
    Sturdy,
    SwoopingFlyer,
    Terrifying,
}

static_gen_fn_all!(Trait);

impl std::fmt::Display for Trait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_case(Case::Lower))
    }
}

impl Trait {
    pub fn description(&self) -> String {
        match self {
            Self::Mutation => format!("Has the listed mutations."),

            Self::Addicted => format!(
                "Addicted to a specific substance. When this character takes a day rest, they \
                 must consume it or temporarily reduce WIT by 1. After a full rest, they must \
                 consume 4 units or temporarily reduce WIT by 1 for each missing unit."
            ),

            Self::Frightening => format!(
                "Causes fear within range 4. Is immune to ~{}~ characters and treat ~{}~ \
                 characters as ~{}~.",
                Trait::Frightening,
                Trait::Terrifying,
                Trait::Frightening,
            ),
            Self::Terrifying => format!(
                "Causes terror within range 4. Is immune to ~{}~ and ~{}~ characters.",
                Trait::Frightening,
                Trait::Terrifying,
            ),

            Self::Immobile => format!("Can't move."),
            Self::Slow => format!("Moves at half speed."),
            Self::Fast => format!("Moves at double speed."),

            Self::LandingFlyer => {
                format!("Can fly. Can't attack and can't stay still while flying.")
            }
            Self::SwoopingFlyer => {
                format!("Can fly. Can attack but can't stay still while flying.")
            }
            Self::HoveringFlyer => format!("Can fly. Can attack and can stay still while flying."),

            Self::Incorporeal => format!(
                "Has no STR and AGI score. Can't interact with the material world but is still \
                 bound by its laws. Can walk through thin barriers, such as doors."
            ),
            Self::Frail => format!("Halve carry limit and bulk."),
            Self::Sturdy => format!("Double carry limit and bulk."),

            Self::Regeneration => format!(
                "Recovers half the amount of lost health each round, at the start of their turn. \
                 Fire prevents this."
            ),
        }
    }
}
