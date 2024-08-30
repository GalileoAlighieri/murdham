use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Skill {
    Acrobatics,
    Acting,
    Alchemy,
    Ambidexterity,
    AnimalHandling,
    Apothecary,
    Augury,
    Bargaining,
    BattleFrenzy,
    Blathering,
    Boatmanship,
    Brawling,
    Burglary,
    Bushcraft,
    Charm,
    CleavingStrike,
    Climbing,
    Contortionism,
    Crafting,
    Disarming,
    DiseaseResistance,
    Divination,
    DrivingCarts,
    Erudition,
    FastAttack,
    FastDodge,
    Fearless,
    FireEating,
    Frugality,
    Gossiping,
    Healing,
    Hunting,
    Languages,
    Leadership,
    LethalAttack,
    Luck,
    MagicSense,
    MagicShield,
    Medicine,
    Meditating,
    MonsterSlaying,
    Music,
    PiercingStrike,
    PlayingGames,
    PoisonResistance,
    QuickDraw,
    Religion,
    Riding,
    Running,
    ShieldMastery,
    SkilledShot,
    SkilledStrike,
    SneakAttack,
    Sneaking,
    Sorcery,
    SteadyAim,
    Stealing,
    StrikeToStun,
    Swimming,
    Wrestling,
}

static_gen_fn_all!(Skill);

impl Skill {
    pub fn description(&self) -> String {
        match self {
            // Movement
            Self::Running => format!(
                "When you sprint, you can move by an additional zone. You can sprint for two \
                 consecutive stretches without needing to pass an AGI save, and for a third one \
                 by passing an AGI save. You can freely dodge attacks of opportunity, without \
                 needing to spend your turn.",
            ),
            Self::Swimming => format!(
                "Pick one while swimming: move at full speed, automatically pass saves to avoid \
                 drowning, swim in almost impossible circumstances. Your attacks aren't impaired \
                 while swimming. You can hold your breath for twice as long."
            ),
            Self::Climbing => format!(
                "Pick one while climbing: move at full speed, automatically pass saves to avoid \
                 falling, climb an almost impossible surface."
            ),
            Self::Acrobatics => format!(
                "Pick one while moving on difficult ground: move at full speed, automatically \
                 pass saves to avoid falling, balance on extremely difficult terrain. You \
                 automatically pass saves to leap normal distances, and can leap almost \
                 impossible distances by passing a save. When you fall, you reduce the falling \
                 distance by 4 metres if you pass the AGI save, and by 2 metres even if you fail.",
            ),

            Self::DrivingCarts => format!(
                "Pick one while driving a cart: automatically pass a save to prevent the cart \
                 from toppling, drive in almost impossible situations. When your cart sprints, \
                 you can still use your main action to attack, but it must be done after the \
                 movement is fully completed.",
            ),
            Self::Boatmanship => format!(
                "You count as two people when rowing a boat and know how to sail. When you \
                 forage, on a ~fish~ result you find an additional ~{}~, even if you don't have \
                 ~{}~.",
                ItemKind::Ration,
                ItemKind::FishingTools
            ),
            Self::Riding => format!(
                "Pick one while riding: ride without a ~{}~, automatically pass saves to avoid \
                 falling, ride in almost impossible situations, ride an untamed beast. When your \
                 mount sprints, you can still use your main action to attack, but it must be done \
                 after the movement is fully completed.",
                ItemKind::Saddle
            ),

            Self::Contortionism => format!(
                "Your joints are extremely flexible and you can bend your body in absurd shapes. \
                 You can squeeze through small openings and easily escape bonds.",
            ),

            // Stealth and alertness
            Self::Burglary => format!(
                "Pick one when opening a lock or forcing something open: do it without tools \
                 (~{}~ or ~{}~), do it silently, or do it in only one round if you pass the save \
                 or in a stretch as usual even if you fail. You can react to traps even when you \
                 are unaware of them.",
                ItemKind::Crowbar,
                ItemKind::LockPicks
            ),
            Self::Sneaking => format!(
                "Pick one while sneaking: move at full speed, automatically pass saves to avoid \
                 being discovered, sneak in almost impossible situations. At the start of an \
                 encounter, if your group was detected but you weren't acting recklessly, make an \
                 AGI save: on a pass you personally weren't noticed. You could exploit this, for \
                 example, to sneak undetected or start a fight concealed.",
            ),
            Self::Stealing => format!(
                "Pick one while picking pockets: automatically pass saves to avoid being \
                 discovered, attempt to steal an item with bulk 1."
            ),

            // Knowledge
            Self::Erudition => format!(
                "You can read, write, and perform advanced calculations. You can speak and \
                 understand Classic, the language of scholars and the Church. You are familiar \
                 with all manners of academic lore: history, geography, beasts, plants, \
                 philosophy, mathematics, astronomy, etc. When your knowledge might come in \
                 handy, the GM you might give you additional information."
            ),
            Self::Languages => format!(
                "You can read and write. Each time you enter into contact with a language there \
                 is chance you know it: 1:2 for common languages, 1:4 for dead or remote \
                 languages."
            ),
            Self::Alchemy => format!(
                "You can read and write. You can speak and understand Classic, the language of \
                 scholars and the Church. You can craft alchemical substances, such as ~{}s~, \
                 ~{}s~, ~{}~, and ~{}s~. This requires raw materials (worth ¼ of the item) and an \
                 alchemist's workshop. You can craft a batch of 2 consumables of the same type in \
                 a watch.",
                ItemKind::AcidVial,
                ItemKind::AlchemistsFire,
                ItemKind::FlashPowder,
                ItemKind::SmokeBomb,
            ),
            Self::Medicine => format!(
                "You can read and write. You can speak and understand Classic, the language of \
                 scholars and the Church. You can diagnose poison and disease by spending a round \
                 examining a victim. After diagnosing, you can instruct someone with the ~{}~ \
                 skill to create a bespoke ~{}~ or ~{}~ which always works against the specific \
                 poison or disease.",
                Skill::Apothecary,
                ItemKind::Antidote,
                ItemKind::Cure,
            ),
            Self::Apothecary => format!(
                "You can craft medicinal substances, such as ~{}~, ~{}~, ~{}~, ~{}~, ~{}~, ~{}~, \
                 and all kinds of poison. This requires suitable ingredients and ~{}~. Creating a \
                 single dose takes only a stretch of time, but ingredients are rare. It takes a \
                 watch to gather them in the wilderness (if they are locally present), or to find \
                 them for sale in a settlement (they are worth ¼ the value of the end product).",
                ItemKind::Antidote,
                ItemKind::Cure,
                ItemKind::Darkroot,
                ItemKind::HealingDraught,
                ItemKind::MadcapMushroom,
                ItemKind::MedicineBox,
                ItemKind::ApothecaryTools
            ),
            Self::Healing => format!(
                "When you use a ~{}~, your patients always recover half their maximum health. You \
                 can alternatively use a ~{}~ spending only a main action instead of a stretch, \
                 in which case however you heal the usual amount. When you use ~{}~, you are \
                 automatically successful without needing to pass a WIT save.",
                ItemKind::MedicineBox,
                ItemKind::MedicineBox,
                ItemKind::SurgeryTools,
            ),

            // Social
            Self::AnimalHandling => format!(
                "You know how to take care of animals: feeding, grooming, taming, training, \
                 recognising signs of discomfort, etc. You can befriend wild animals by offering \
                 food and passing a WIT save, and domesticated animals by doing either. \
                 Befriended animals follow you until the end of the watch or you leave the area \
                 where they live. You can't befriend hostile animals, and you can only be \
                 accompanied by one befriended animal at a time.",
            ),
            Self::Charm => format!(
                "You can befriend and persuade people without a WIT save in challenging \
                 circumstances, and by passing a WIT save in almost impossible circumstances. If \
                 you spend a stretch chatting or observing someone, you are able to estimate if \
                 they are bribable, and what it might be necessary to convince them.",
            ),
            Self::Blathering => format!(
                "You are able to speak endless strings of nonsense, leaving others dumbfounded. \
                 You can distract and taunt people without a WIT save in challenging \
                 circumstances, and by passing a WIT save in almost impossible circumstances."
            ),
            Self::Gossiping => format!(
                "When you take a day or full rest in a settlement, you might hear interesting \
                 rumours. The GM decides what you hear, and it isn't necessarily true. You can \
                 easily find contacts, even illegal ones such as fences, by spending a watch \
                 asking around in a settlement.",
            ),
            Self::Bargaining => format!(
                "You sell items at full price, rather than half price. Other factors which might \
                 reduce the value of an item still apply. You are able to estimate the value of \
                 most items just by examining them.",
            ),
            Self::Leadership => format!(
                "You can inspire, intimidate, and keep the loyalty of retainers, allies, and \
                 other underlings without a WIT save in challenging circumstances, and by passing \
                 a WIT save in almost impossible circumstances. You can spend a main action to \
                 rally all ~{}~ and ~{}~ allies within range 2. They make a group WIT save and \
                 those who succeed recover immediately. Each character can be affected by this \
                 skill only once per stretch.",
                Condition::Frightened,
                Condition::Terrified
            ),
            Self::Acting => format!(
                "You are able to convincingly fake emotions and to disguise your voice and \
                 accent. This might give you an advantage in suitable social interactions or can \
                 be used to complement a disguise.",
            ),
            Self::Music => format!(
                "You know how to sing and play music instruments. During a day rest you can play \
                 an inspiring song for your party: all companions have a 1:4 chance of recovering \
                 1 spent omen."
            ),

            // Magic
            Self::Augury => format!(
                "You can spend a stretch to consult the entrails of a dead medium-sized animal to \
                 gain an omen. Sometimes, the entrails might provide an useful piece of \
                 information, at the GM's discretion.",
            ),
            Self::Divination => format!(
                "If you have ~{}~, you can spend an omen and a stretch of time to ask a question \
                 pertaining your current situation. The GM describes a vision giving you a \
                 cryptic answer. There is a 1:4 chance that the vision is wrong or misleading, \
                 rolled secretly by the GM.",
                ItemKind::DivinationTools,
            ),
            Self::MagicSense => format!(
                "You can spend a stretch in meditation to sense the presence of magic phenomena \
                 (ongoing powers, demons, magical creatures, etc.) in your zone or in your sector \
                 (your choice). You can only detect if any magic phenomena is present in the \
                 area, but can't count them, locate them, or determine their nature.",
            ),
            Self::MagicShield => format!(
                "You can use an ancient technique to erect a magic shield around you. Activating \
                 or deactivating it takes a stretch spent in meditation, and it deactivates \
                 automatically if you are ~{}~ or fall asleep. Sorcerous powers have a 1:2 chance \
                 of not working on you, no matter if harmful or beneficial. Other targets aren't \
                 protected by the shield. Sacred powers aren't affected. Sorcerers can enhance \
                 their powers to ignore the magic shield by increasing their level by 1.",
                Condition::Incapacitated,
            ),
            Self::Religion => format!(
                "You can read and write. You can speak and understand Classic, the language of \
                 scholars and the Church. You can invoke sacred powers. You can't acquire the \
                 ~{}~ skill.",
                Skill::Sorcery
            ),
            Self::Meditating => format!("You heal 1 corruption when you take a day rest."),
            Self::Sorcery => format!(
                "You can read and write. You can speak and understand Magick, the language used \
                 to invoke sorcerous powers. This language is too convoluted to be used to \
                 communicate, but is essential to use magic. You can invoke sorcerous powers. You \
                 can increase your maximum mana by 1 instead of taking a normal advancement, up \
                 to 6 at most. You can't acquire the ~{}~ skill.",
                Skill::Religion
            ),

            // Combat
            Self::BattleFrenzy => format!(
                "You can become ~{}~ by spending a main action or freely when you suffer damage. \
                 The condition lasts until all enemies have been defeated. You can spend a main \
                 action to try to calm yourself by passing a WIT save. While you are frenzied, \
                 you recover 1 health for each enemy you kill.",
                Condition::Frenzied
            ),
            Self::SkilledStrike => format!(
                "You improve the damage die of melee attacks (but not unarmed attacks): d4 to d6, \
                 d6 to d8, d8 to d10, d10 to d12. You can't improve a d12. In case of blast \
                 attacks only one target takes increased damage.",
            ),
            Self::SkilledShot => format!(
                "You improve the damage die of ranged attacks: d4 to d6, d6 to d8, d8 to d10, d10 \
                 to d12. You can't improve a d12. In case of blast attacks only one target takes \
                 increased damage.",
            ),
            Self::SteadyAim => format!("You double the range of ranged attacks."),
            Self::CleavingStrike => format!(
                "When you inflict critical damage or kill a target with a melee attack, you can \
                 immediately attack another target with the same weapon. You can do this at most \
                 once per turn, and this rule doesn't apply to attacks made to counter.",
            ),
            Self::Brawling => format!(
                "Your unarmed attacks are not impaired and inflict d6 damage. Your armour value \
                 is increased by 1 against unarmed attacks.",
            ),
            Self::Wrestling => format!(
                "When you perform a grapple attack, your target can't resist with a STR save \
                 unless they also have this skill. Only characters with this skill can grapple \
                 you.",
            ),
            Self::Ambidexterity => format!(
                "You can use both hands equally well. Damage is not impaired when you attack with \
                 a weapon in your non-dominant hand. You can attack the same target with two \
                 weapons at once, rolling damage for both but only considering the higher roll.",
            ),
            Self::Disarming => format!(
                "When you perform a disarm attack, your target can't resist with a STR save \
                 unless they also have this skill. Only characters with this skill can disarm you.",
            ),
            Self::SneakAttack => format!(
                "Your attacks against unaware enemies always inflict d12 damage, no matter what \
                 weapons you use or if you are unarmed. Unarmed attacks still inflict impaired \
                 damage.",
            ),
            Self::StrikeToStun => format!(
                "When you attack an enemy, you may choose to perform a stunning blow. The attack \
                 inflicts no damage but you must still roll for damage. If you roll equal or \
                 greater than half the target's remaining health, they are ~{}~ until the end of \
                 the stretch. If you roll equal or greater than their whole remaining health, \
                 they are ~{}~ until the end of the watch.",
                Condition::Incapacitated,
                Condition::Incapacitated
            ),
            Self::FastDodge => format!(
                "Once per round, you can dodge an attack without spending your turn, even if you \
                 have already acted. You can't dodge the same attack twice, but you can dodge and \
                 counter the same attack, and you can dodge when your attack is countered. You \
                 can use this skill to dodge an opportunity attack or while guarding.",
            ),
            Self::FastAttack => format!(
                "When you counter an attack or your attack is countered you always hit first \
                 unless your opponent also has this skill.",
            ),
            Self::MonsterSlaying => {
                format!("You inflict double damage against targets larger than you.")
            }
            Self::LethalAttack => format!(
                "When you inflict critical damage with an attack, you may choose to instantly \
                 kill the target."
            ),
            Self::PiercingStrike => format!(
                "If you roll higher than the target's armour value with a melee weapon (not \
                 unarmed attacks), you inflict full damage. If you roll equal or lower, you still \
                 inflict no damage.",
            ),
            Self::QuickDraw => {
                format!(
                    "You can equip and unequip any number of items held in hand as a single bonus \
                     action."
                )
            }
            Self::ShieldMastery => {
                format!(
                    "When you hold a shield, your armour value is increased by 1 against all \
                     attacks, not just if you dodge, counter, or are countered. If you are \
                     unaware of the attack, however, your shield still doesn't protect you."
                )
            }

            // Miscellanea
            Self::FireEating => format!(
                "You can consume ~{}~ and spit it through an open flame, such as a lit ~{}~, to \
                 make a melee blast attack inflicting d4 fire damage. You reduce incoming fire \
                 damage by 1.",
                ItemKind::AlcoholicDrink,
                ItemKind::Torch
            ),
            Self::Frugality => format!(
                "You don't reduce abilities when you can't satisfy needs during a day rest. \
                 However, you still have to satisfy them all in order to heal. You pay half for \
                 lodging, as your standards are very low and are content with little."
            ),
            Self::Luck => format!(
                "Your maximum omens are increased by 1. When you spend an omen, there is a 1:4 \
                 chance it isn't actually spent. When choosing the target of an indiscriminate \
                 effect, such as a trap or a monster ambush, the GM might prioritise other \
                 characters over you.",
            ),
            Self::PlayingGames => format!(
                "You can learn to play games quickly: after you have played a game, you can't be \
                 beaten by others unless they also have this skill. You know how to cheat: your \
                 cheating attempts are always successful unless your opponents are paying close \
                 attention to you. People might still get suspicious if you win too much."
            ),
            Self::Crafting => format!(
                "Pick one when repairing an item: do it without a ~{}~, or do it without having \
                 to pass a save to succeed. You can craft non-consumable items, such as weapons, \
                 armour, and vehicles. This takes a watch and requires raw materials (worth ¼ of \
                 the item) and a workshop with all the necessary tools and equipment. Items with \
                 the ~durability~ keyword require one watch every 2 points of durability to be \
                 crafted.",
                ItemKind::Toolbox
            ),
            Self::Fearless => format!("You are immune to fear and treat terror as fear",),
            Self::PoisonResistance => format!(
                "You are resistant to alcohol, poisons, and drugs. You ignore the first dose \
                 taken within a stretch. You can resist a second dose with a STR save, and a \
                 third dose works automatically.",
            ),
            Self::DiseaseResistance => format!(
                "After you recover from a disease, you become immune to it and can't contract it \
                 again."
            ),
            Self::Bushcraft => format!(
                "Pass a WIT save to ignore the movement penalty when travelling between sectors \
                 or regions without following a path. If you have ~{}~, you pass automatically. \
                 You can sleep in the wilderness without a ~{}~. When you forage, you roll twice \
                 on the foraging table and apply both results.",
                ItemKind::NavigationTools,
                ItemKind::CampingKit,
            ),
            Self::Hunting => format!(
                "You automatically follow trails without a WIT save in normally challenging \
                 circumstances, and by passing a WIT save in almost impossible situations. When \
                 you forage, on a ~small game~ result you find an additional ~{}~, even if you \
                 don't have ~{}~.",
                ItemKind::Ration,
                ItemKind::TrappingTools,
            ),
        }
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_case(Case::Lower))
    }
}
