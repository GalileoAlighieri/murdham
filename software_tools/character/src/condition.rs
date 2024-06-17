use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::{utils::*, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Condition {
    ArmInjury,
    Blind,
    Confused,
    Deaf,
    Dying,
    Encumbered,
    Entangled,
    EyeInjury,
    Frenzied,
    Frightened,
    Hobbled,
    Incapacitated,
    Poisoned,
    Sick,
    Silenced,
    Stunned,
    Terrified,
}

static_gen_fn_all!(Condition);

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_case(Case::Lower))
    }
}

impl Condition {
    pub fn description(&self) -> String {
        match self {
            Self::ArmInjury => format!("You can't use one of your arms."),
            Self::Blind => format!(
                "You can't see. Enemies are not visible by you (with associated penalties to \
                 attacks) and you can't notice anything based on sight alone.",
            ),
            Self::Confused => format!(
                "You are not fully in control of your body and mind. Your attacks inflict \
                 impaired damage and your abilities are temporarily reduced by 2. When you \
                 recover from this condition, your abilities are increased back by 2. If you \
                 become ~{}~ again while you already are, you become ~{}~ instead.",
                Condition::Confused,
                Condition::Incapacitated,
            ),
            Self::Deaf => format!(
                "You can't hear. You can't notice anything based on sound alone and you might \
                 have trouble communicating with others or noticing ambushes.",
            ),
            Self::Dying => format!(
                "You die at the end of the next round. If you recover at least 1 health before \
                 then you are stabilised but suffer an injury.",
            ),
            Self::Encumbered => format!(
                "You move at half speed and automatically fail AGI saves. You recover as soon as \
                 the total bulk you are carrying doesn't exceed your STR.",
            ),
            Self::Entangled => format!(
                "You are unable to act. You can only spend your turn to try to break free by \
                 passing a STR save. Nearby characters can attempt to free you by spending a main \
                 action and passing a STR save.",
            ),
            Self::EyeInjury => format!(
                "You are unable to see from one of your eyes. You halve the range of ranged \
                 attacks and powers and activities requiring perfect sight or depth perception, \
                 such as leaping, might be more challenging.",
            ),
            Self::Frenzied => format!(
                "You can only spend your turn to attack, or if that's not possible to move as \
                 close as possible to the closest enemy in preparation for a future attack. You \
                 can't retreat or stop fighting. You are immune to fear, treat terror as fear, \
                 and you aren't ~{}~ when you suffer critical damage.",
                Condition::Incapacitated,
            ),
            Self::Frightened => {
                format!("You are scared by something. You can't approach the source of your fear.",)
            }
            Self::Hobbled => format!(
                "Your movement speed is halved and you must use a stick or crutch to stand.",
            ),
            Self::Incapacitated => format!(
                "You are unconscious or in terrible pain. You are completely unable to act and \
                 defenceless.",
            ),
            Self::Poisoned => {
                format!(
                    "You suffer the effects of a poison unless you take an {} in time. See the \
                     relevant section for more details.",
                    ItemKind::Antidote
                )
            }
            Self::Sick => format!(
                "You must pass a STR save each day to avoid debilitating effects and to recover. \
                 See the relevant section for more details.",
            ),
            Self::Silenced => format!(
                "You can't speak or emit sounds. Among other things, this renders you unable to \
                 invoke powers."
            ),
            Self::Stunned => format!(
                "You are temporarily unable to act: you might be dazed, thrown on the ground, \
                 etc. You can't act for a round. In combat, this means you lose your next turn, \
                 either on the current round or on the next one (if you have already acted this \
                 round).",
            ),
            Self::Terrified => format!(
                "You are terrified by something. You must run away from the source of your \
                 terror, or if that's not possible you cower down and are considered ~{}~.",
                Condition::Incapacitated,
            ),
        }
    }
}
