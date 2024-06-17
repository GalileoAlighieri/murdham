use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum PowerKind {
    Sacred,
    Sorcerous,
}

static_gen_fn_all!(PowerKind);

impl std::fmt::Display for PowerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_case(Case::Lower))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum PowerDuration {
    Instant,
    Stretch,
    Watch,
    Lingering,
}

static_gen_fn_all!(PowerDuration);

impl std::fmt::Display for PowerDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_case(Case::Lower))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum PowerRange {
    Personal,
    Near,
    Sight,
    Connection,
}

static_gen_fn_all!(PowerRange);

impl std::fmt::Display for PowerRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_case(Case::Lower))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Power {
    // Sacred
    RiteOfBane,
    RiteOfBlessing,
    RiteOfCourage,
    RiteOfHealing,
    RiteOfJudgement,
    RiteOfMartyrdom,
    RiteOfPreservation,
    RiteOfProtection,
    RiteOfPurging,
    RiteOfPurification,
    RiteOfRestoration,
    RiteOfWarding,

    // Sorcerous
    EldritchBlast,

    ArcaneLock,
    Bewitch,
    BurningVengeance,
    GiftOfSpeech,
    Illusion,
    Levitation,
    MiasmaOfChaos,
    OccultConsultation,
    PlagueVessel,
    PurifyingFlame,
    ReanimateCorpse,
    WaterBreathing,
}

static_gen_fn_all!(Power);

impl Power {
    pub fn kind(&self) -> PowerKind {
        match self {
            Self::RiteOfBane => PowerKind::Sacred,
            Self::RiteOfBlessing => PowerKind::Sacred,
            Self::RiteOfCourage => PowerKind::Sacred,
            Self::RiteOfHealing => PowerKind::Sacred,
            Self::RiteOfJudgement => PowerKind::Sacred,
            Self::RiteOfMartyrdom => PowerKind::Sacred,
            Self::RiteOfPreservation => PowerKind::Sacred,
            Self::RiteOfProtection => PowerKind::Sacred,
            Self::RiteOfPurging => PowerKind::Sacred,
            Self::RiteOfPurification => PowerKind::Sacred,
            Self::RiteOfRestoration => PowerKind::Sacred,
            Self::RiteOfWarding => PowerKind::Sacred,

            Self::EldritchBlast => PowerKind::Sorcerous,

            Self::ArcaneLock => PowerKind::Sorcerous,
            Self::Bewitch => PowerKind::Sorcerous,
            Self::BurningVengeance => PowerKind::Sorcerous,
            Self::GiftOfSpeech => PowerKind::Sorcerous,
            Self::Illusion => PowerKind::Sorcerous,
            Self::Levitation => PowerKind::Sorcerous,
            Self::MiasmaOfChaos => PowerKind::Sorcerous,
            Self::OccultConsultation => PowerKind::Sorcerous,
            Self::PlagueVessel => PowerKind::Sorcerous,
            Self::PurifyingFlame => PowerKind::Sorcerous,
            Self::ReanimateCorpse => PowerKind::Sorcerous,
            Self::WaterBreathing => PowerKind::Sorcerous,
        }
    }

    pub fn range(&self) -> PowerRange {
        match self {
            Self::RiteOfBane => PowerRange::Near,
            Self::RiteOfBlessing => PowerRange::Near,
            Self::RiteOfCourage => PowerRange::Near,
            Self::RiteOfHealing => PowerRange::Near,
            Self::RiteOfJudgement => PowerRange::Sight,
            Self::RiteOfMartyrdom => PowerRange::Near,
            Self::RiteOfPreservation => PowerRange::Near,
            Self::RiteOfProtection => PowerRange::Near,
            Self::RiteOfPurging => PowerRange::Near,
            Self::RiteOfPurification => PowerRange::Near,
            Self::RiteOfRestoration => PowerRange::Near,
            Self::RiteOfWarding => PowerRange::Near,

            Self::EldritchBlast => PowerRange::Near,

            Self::ArcaneLock => PowerRange::Near,
            Self::Bewitch => PowerRange::Near,
            Self::BurningVengeance => PowerRange::Near,
            Self::GiftOfSpeech => PowerRange::Near,
            Self::Illusion => PowerRange::Near,
            Self::Levitation => PowerRange::Near,
            Self::MiasmaOfChaos => PowerRange::Near,
            Self::OccultConsultation => PowerRange::Near,
            Self::PlagueVessel => PowerRange::Near,
            Self::PurifyingFlame => PowerRange::Near,
            Self::ReanimateCorpse => PowerRange::Near,
            Self::WaterBreathing => PowerRange::Near,
        }
    }

    pub fn duration(&self) -> PowerDuration {
        match self {
            Self::RiteOfBane => PowerDuration::Watch,
            Self::RiteOfBlessing => PowerDuration::Lingering,
            Self::RiteOfCourage => PowerDuration::Watch,
            Self::RiteOfHealing => PowerDuration::Instant,
            Self::RiteOfJudgement => PowerDuration::Instant,
            Self::RiteOfMartyrdom => PowerDuration::Watch,
            Self::RiteOfPreservation => PowerDuration::Lingering,
            Self::RiteOfProtection => PowerDuration::Watch,
            Self::RiteOfPurging => PowerDuration::Instant,
            Self::RiteOfPurification => PowerDuration::Instant,
            Self::RiteOfRestoration => PowerDuration::Instant,
            Self::RiteOfWarding => PowerDuration::Watch,

            Self::EldritchBlast => PowerDuration::Instant,

            Self::ArcaneLock => PowerDuration::Stretch,
            Self::Bewitch => PowerDuration::Stretch,
            Self::BurningVengeance => PowerDuration::Stretch,
            Self::GiftOfSpeech => PowerDuration::Stretch,
            Self::Illusion => PowerDuration::Stretch,
            Self::Levitation => PowerDuration::Stretch,
            Self::MiasmaOfChaos => PowerDuration::Stretch,
            Self::OccultConsultation => PowerDuration::Stretch,
            Self::PlagueVessel => PowerDuration::Stretch,
            Self::PurifyingFlame => PowerDuration::Stretch,
            Self::ReanimateCorpse => PowerDuration::Stretch,
            Self::WaterBreathing => PowerDuration::Stretch,
        }
    }

    pub fn description(&self) -> String {
        match self {
            // Sacred
            Self::RiteOfBane => format!(
                "Target an item. It burns {}s and {} touching it. If it's a weapon or a shield, \
                 it inflicts enhanced damage against them. If it's armour or a shield, {}s and {} \
                 attacking the wearer and not overcoming their armour value suffer direct damage \
                 equal to what they rolled.",
                CharacterCategory::Demon,
                CharacterCategory::Undead,
                CharacterCategory::Demon,
                CharacterCategory::Undead
            ),
            Self::RiteOfBlessing => format!(
                "Target 2 bottles filled with clean, pure water. They are turned into ~{}~.",
                ItemKind::BlessedWater
            ),
            Self::RiteOfCourage => format!(
                "Target a character with {}. The target emanates an aura which fills their allies \
                 with confidence and calm. The target and all nearby allies are immune to fear \
                 and treat terror as fear.",
                IntelligenceCategory::HumanIntelligence,
            ),
            Self::RiteOfHealing => format!(
                "Target a {}. The target heals damage equal to half their STR.",
                CharacterCategory::Creature
            ),
            Self::RiteOfJudgement => format!(
                "Target a {} with {} and accuse them of a grave injustice. The target is engulfed \
                 by flames: they are reduced to ashes if guilty, but left unharmed if innocent. \
                 This power doesn't work with minor accusations of little significance.",
                CharacterCategory::Creature,
                IntelligenceCategory::HumanIntelligence
            ),
            Self::RiteOfMartyrdom => format!(
                "Target two {}s. When one of the two targets suffer damage, half of it is \
                 suffered by the other target instead.",
                CharacterCategory::Creature
            ),
            Self::RiteOfPreservation => format!(
                "Target a corpse. It can't be affected by sorcerous powers and it decays about \
                 1,000 times more slowly than normal.",
            ),
            Self::RiteOfProtection => format!(
                "Target a character. The target's armour value increases by 1, but only if they \
                 don't wear armour. They must show their faith in divine protection! Shields are \
                 allowed.",
            ),
            Self::RiteOfPurging => format!(
                "Target an item which has been tampered with sorcerous magic, a {}, an {}, or a \
                 character who has suffered mutations. The target must remain within range for \
                 the whole time required to invoke the power. If successful, the target is \
                 banished, destroyed, or killed.",
                CharacterCategory::Demon,
                CharacterCategory::Undead,
            ),
            Self::RiteOfPurification => format!(
                "Target a {}. The target heals 1 corruption.",
                CharacterCategory::Creature,
            ),
            Self::RiteOfRestoration => format!(
                "Target a {} who is ~{}~ or affected by poison. Poison is neutralised, and ~{}~ \
                 targets gain a success towards recovering.",
                CharacterCategory::Creature,
                Condition::Sick,
                Condition::Sick,
            ),
            Self::RiteOfWarding => format!(
                "You erect an anti-magic field covering all zones within range 1 of your current \
                 position. Within the area, sorcerous powers have no effect and {}s and {} suffer \
                 d4 direct damage per round. The field is fixed and doesn't move with you.",
                CharacterCategory::Demon,
                CharacterCategory::Undead,
            ),

            // Sorcerous - Basic
            Self::EldritchBlast => format!(
                "Target a character or an item. The target is hit by a force blast, which works \
                 as a melee attack inflicting d10 damage (impaired against items). If you enhance \
                 the power to ~{}~ range, it works as a ranged attack instead. All skills and \
                 rules concerning attacks apply, and the attack can be dodged or countered as \
                 usual. If you enhance the power to ~{}~ range, the blast is conjured on top of \
                 the target and your skills don't apply any longer, but the target can still \
                 dodge.",
                PowerRange::Sight,
                PowerRange::Connection
            ),

            // Sorcerous - Advanced
            Self::ArcaneLock => format!(
                "Target an item which can be opened and closed, such as a door, a box, or a \
                 bottle. The target can only be opened by a trigger of your choice (a password, a \
                 gesture, contact with a specific item, etc."
            ),
            Self::Bewitch => format!(
                "Target a {}. The target attitude towards you improves by a step.",
                CharacterCategory::Creature
            ),
            Self::BurningVengeance => format!(
                "Target a {} with {}. Name a person they know and remind how they have been \
                 wronged by them, no matter if by a small or a big matter. The target becomes \
                 obsessed with an irrational desire to take revenge by killing the named person \
                 and they will fully devote themselves to this end.",
                CharacterCategory::Creature,
                IntelligenceCategory::HumanIntelligence,
            ),
            Self::GiftOfSpeech => format!(
                "Target a {} with {} or {}. The target becomes able to speak your language.",
                CharacterCategory::Creature,
                IntelligenceCategory::AnimalIntelligence,
                IntelligenceCategory::HumanIntelligence,
            ),
            Self::Illusion => format!(
                "Target a small zone. You can create an illusion within its boundaries. The \
                 illusion can affect sight, hearing, and smell, but not touch. While you are \
                 within range, you can concentrate on the illusion to change it or animate it. \
                 While you aren't concentrating the illusion remains frozen in the state you left \
                 it, and only disappears when the power ends."
            ),
            Self::Levitation => format!(
                "Target a {}. The target temporarily gains the ~{}~ trait.",
                CharacterCategory::Creature,
                Trait::LandingFlyer,
            ),
            Self::MiasmaOfChaos => format!(
                "Target a small zone. The area is filled with red mist flowing from the ground. \
                 Characters entering or starting their turn in the affected area suffer 1 \
                 corruption. The mist blocks visibility through it, but it still possible to see \
                 nearby characters within it."
            ),
            Self::OccultConsultation => format!(
                "Target a corpse of a {} who has died no longer than a stretch ago and whose \
                 spirit is still lingering in the area. You may ask their spirit one question, \
                 but they aren't forced to answer or tell the truth. If you enhance the power \
                 range to ~{}~, you may query the spirit of a long dead person who is already in \
                 the realm of the dead, but in they can only answer by knocking on its entrance. \
                 This means they can only answer with a number or a yes-or-no question.",
                CharacterCategory::Creature,
                PowerRange::Connection
            ),
            Self::PlagueVessel => format!(
                "Target a ~{}~ {}. You absorb the disease from the target, who is instantly \
                 healed. You carry the disease in your body without suffering its effect. You can \
                 attempt to discharge it by touching another {} before the power ends. The target \
                 can resist the infection with a STR save, if normally possible. If they resist, \
                 the disease stays in your body, and you can attempt to discharge it again later. \
                 If the power ends before you discharge the disease, you are infected by it \
                 yourself, without chance of a save.",
                Condition::Sick,
                CharacterCategory::Creature,
                CharacterCategory::Creature,
            ),
            Self::PurifyingFlame => format!(
                "Target an unlit light source, such as a ~{}~, an ~{}~, a ~{}~, or a bonfire. It \
                 is instantly lit with a bright white flame. All {}s illuminated by it are immune \
                 to poison and disease. If they are already poisoned, the poison isn't \
                 neutralised but its effect is suspended while under the light. If they are \
                 already ~{}~, they aren't healed but don't suffer negative effects if they fail \
                 a recovery roll while under the light.",
                ItemKind::Candle,
                ItemKind::OilLamp,
                ItemKind::Torch,
                CharacterCategory::Creature,
                Condition::Sick,
            ),
            Self::ReanimateCorpse => {
                format!(
                    "Target the corpse of a {} (or smaller) {}. It is raised as a reanimated \
                     corpse under your control.",
                    SizeCategory::MediumSized,
                    CharacterCategory::Creature
                )
            }
            Self::WaterBreathing => format!(
                "Target a {}. Gills and fins appear on the target's body and their hands, feet, \
                 or paws become palmate. The target can breathe under water and can swim at full \
                 speed.",
                CharacterCategory::Creature
            ),
        }
    }

    pub fn enhancements(&self) -> Vec<(i8, String)> {
        match self {
            Self::RiteOfBane => Vec::new(),
            Self::RiteOfBlessing => Vec::new(),
            Self::RiteOfCourage => Vec::new(),
            Self::RiteOfHealing => Vec::new(),
            Self::RiteOfJudgement => Vec::new(),
            Self::RiteOfMartyrdom => Vec::new(),
            Self::RiteOfPreservation => Vec::new(),
            Self::RiteOfProtection => Vec::new(),
            Self::RiteOfPurging => Vec::new(),
            Self::RiteOfPurification => Vec::new(),
            Self::RiteOfRestoration => Vec::new(),
            Self::RiteOfWarding => Vec::new(),

            Self::EldritchBlast => vec![(
                1,
                format!("The power inflicts fire, lightning, cold, or heat damage."),
            )],

            Self::ArcaneLock => vec![(
                1,
                format!(
                    "The target emits a loud sound if someone attempts to open it without using \
                     the proper method."
                ),
            )],
            Self::Bewitch => vec![
                (1, format!("Improve the attitude by an additional step")),
                (
                    1,
                    format!(
                        "Target another {}, who replaces you as the one towards whom attitude is \
                         improved.",
                        CharacterCategory::Creature
                    ),
                ),
            ],
            Self::BurningVengeance => Vec::new(),
            Self::GiftOfSpeech => Vec::new(),
            Self::Illusion => Vec::new(),
            Self::Levitation => vec![(
                1,
                format!(
                    "Improve flying by a category: ~{}~ to ~{}~, ~{}~ to ~{}~.",
                    Trait::LandingFlyer,
                    Trait::SwoopingFlyer,
                    Trait::SwoopingFlyer,
                    Trait::HoveringFlyer,
                ),
            )],
            Self::MiasmaOfChaos => vec![
                (1, format!("Double the amount of corruption.")),
                (
                    1,
                    format!(
                        "The mist covers all zones within range 1 of the targeted one. Enhance \
                         again to double the range."
                    ),
                ),
            ],
            Self::PlagueVessel => vec![(
                1,
                format!(
                    "Target another {}, who replaces you as the carrier of the disease.",
                    CharacterCategory::Creature
                ),
            )],

            Self::ReanimateCorpse => vec![(
                2,
                format!("Increase the maximum size category of the targeted corpse by one step."),
            )],

            Self::PurifyingFlame => Vec::new(),
            Self::OccultConsultation => Vec::new(),
            Self::WaterBreathing => Vec::new(),
        }
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_case(Case::Lower))
    }
}

static_gen_fn!(sacred_powers, gen_sacred_powers, Power, Power, {
    vec![
        Self::RiteOfBane,
        Self::RiteOfBlessing,
        Self::RiteOfCourage,
        Self::RiteOfHealing,
        Self::RiteOfJudgement,
        Self::RiteOfMartyrdom,
        Self::RiteOfPreservation,
        Self::RiteOfProtection,
        Self::RiteOfPurging,
        Self::RiteOfPurification,
        Self::RiteOfRestoration,
        Self::RiteOfWarding,
    ]
});

static_gen_fn!(sorcerous_powers, gen_sorcerous_powers, Power, Power, {
    vec![
        Self::ArcaneLock,
        Self::Bewitch,
        Self::BurningVengeance,
        Self::EldritchBlast,
        Self::GiftOfSpeech,
        Self::Illusion,
        Self::Levitation,
        Self::MiasmaOfChaos,
        Self::OccultConsultation,
        Self::PlagueVessel,
        Self::PurifyingFlame,
        Self::ReanimateCorpse,
        Self::WaterBreathing,
    ]
});

static_gen_fn!(
    advanced_sorcerous_powers,
    gen_advanced_sorcerous_powers,
    Power,
    Power,
    {
        vec![
            Self::ArcaneLock,
            Self::Bewitch,
            Self::BurningVengeance,
            Self::GiftOfSpeech,
            Self::Illusion,
            Self::Levitation,
            Self::MiasmaOfChaos,
            Self::OccultConsultation,
            Self::PlagueVessel,
            Self::PurifyingFlame,
            Self::ReanimateCorpse,
            Self::WaterBreathing,
        ]
    }
);
