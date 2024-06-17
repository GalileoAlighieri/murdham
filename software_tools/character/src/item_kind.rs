use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PowerContent {
    Undefined,
    Power(Power),
    PowerOfKind(PowerKind),
}

impl Default for PowerContent {
    fn default() -> Self {
        Self::Undefined
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum ItemKind {
    // Placeholder for anything
    Miscellanea,

    // First for sorting purposes
    Clothes,

    // Armour
    LightArmour,
    HeavyArmour,
    Shield,
    Helmet,

    // Standard weapons
    SimpleHandWeapon,
    SimpleGreatWeapon,
    MartialHandWeapon,
    MartialGreatWeapon,
    MissileHandWeapon,
    MissileGreatWeapon,
    Handgun,
    LongGun,

    // Special weapons
    Garrotte,
    Mancatcher,
    AcidVial,
    AlchemistsFire,
    FlashPowder,
    BlessedWater,

    // Artillery
    Ballista,
    Catapult,
    Cannon,
    Mortar,

    // Vehicles
    Cart,
    Coach,
    RowingBoat,
    SailingBoat,

    // Gear
    AlcoholicDrink,
    AnimalSkin,
    Antidote,
    ApothecaryTools,
    BagOfDung,
    BearTrap,
    BeastmanBlood,
    BlackAdderVenom,
    Book,
    BrushAndPaints,
    ButterflySting,
    Cage,
    Caltrops,
    CampingKit,
    Candle,
    CandlestickRoot,
    Canvas,
    Cards,
    Censer,
    Chain,
    Chalk,
    ChessSet,
    ChimeraSpit,
    ClimbingGear,
    CorpseMandrake,
    CrimsonWeed,
    Crowbar,
    Crutch,
    Cure,
    Darkroot,
    Dice,
    DivinationTools,
    DreamSand,
    FieldKitchen,
    FishingTools,
    FlintAndTinder,
    Gemstone,
    GoldJewel,
    GrapplingHook,
    Grimoire,
    HealingDraught,
    Incense,
    Ladder,
    Lantern,
    LockboxAndKey,
    LockPicks,
    LookingGlass,
    LuckyCharm,
    MadcapMushroom,
    ManaclesAndKey,
    MedicineBox,
    MetalFile,
    Mirror,
    MusicInstrument,
    Muzzle,
    NavigationTools,
    Net,
    OilLamp,
    Painting,
    Perfume,
    PipeAndTobacco,
    PowerScroll(PowerContent),
    Prosthesis,
    QuillAndInk,
    Ration,
    ReapersSpice,
    Rope,
    RopeLadder,
    Saddle,
    Scissors,
    ScorpionOil,
    Scroll,
    ScrollCase,
    SealingWax,
    SignetRing,
    SilverJewel,
    SmokeBomb,
    SurgeryTools,
    Toolbox,
    Torch,
    TrappingTools,
    WarmClothes,
    Water,

    // Assets not in list
    SimpleHugeWeapon,
}

static_gen_fn_all!(ItemKind);

impl std::fmt::Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LockboxAndKey => write!(f, "lockbox & key"),
            Self::ManaclesAndKey => write!(f, "manacles & key"),
            Self::BrushAndPaints => write!(f, "brush & paints"),
            Self::QuillAndInk => write!(f, "quill & ink"),
            Self::FlintAndTinder => write!(f, "flint & tinder"),
            Self::Cure => write!(f, "cure-all"),
            Self::PipeAndTobacco => write!(f, "pipe & tobacco"),
            Self::PowerScroll(x) => match x {
                PowerContent::Undefined => write!(f, "power scroll"),
                PowerContent::Power(x) => write!(f, "scroll of '`{x}`'"),
                PowerContent::PowerOfKind(x) => write!(f, "{x} power scroll"),
            },
            Self::AlchemistsFire => write!(f, "alchemist's fire"),
            Self::ReapersSpice => write!(f, "reaper's spice"),
            _ => write!(f, "{}", format!("{self:?}").to_case(Case::Lower)),
        }
    }
}

impl ItemKind {
    pub fn cost(&self) -> i32 {
        match self {
            Self::Miscellanea => 0,

            // Armour
            Self::LightArmour => 256,
            Self::HeavyArmour => 1024,
            Self::Shield => 64,
            Self::Helmet => 16,

            // Standard weapons
            Self::SimpleHandWeapon => 4,
            Self::SimpleGreatWeapon => 8,
            Self::MartialHandWeapon => 16,
            Self::MartialGreatWeapon => 32,
            Self::MissileHandWeapon => 16,
            Self::MissileGreatWeapon => 32,
            Self::Handgun => 32,
            Self::LongGun => 64,

            // Special weapons
            Self::AcidVial => 4,
            Self::BlessedWater => 4,
            Self::FlashPowder => 16,
            Self::AlchemistsFire => 16,
            Self::SmokeBomb => 16,
            Self::Garrotte => 16,
            Self::Mancatcher => 16,

            // Artillery
            Self::Ballista => 256,
            Self::Catapult => 256,
            Self::Cannon => 1024,
            Self::Mortar => 1024,

            // Gear
            Self::AlcoholicDrink => 4,
            Self::AnimalSkin => 4,
            Self::Antidote => 16,
            Self::ApothecaryTools => 16,
            Self::BagOfDung => 1,
            Self::BearTrap => 16,
            Self::BeastmanBlood => 16,
            Self::BlackAdderVenom => 16,
            Self::Book => 16,
            Self::Scroll => 1,
            Self::BrushAndPaints => 16,
            Self::ButterflySting => 16,
            Self::Cage => 16,
            Self::Caltrops => 16,
            Self::CampingKit => 16,
            Self::Candle => 1,
            Self::CandlestickRoot => 16,
            Self::Canvas => 16,
            Self::Cards => 4,
            Self::Cart => 256,
            Self::Censer => 16,
            Self::Coach => 1024,
            Self::Chain => 4,
            Self::Chalk => 1,
            Self::ChessSet => 16,
            Self::ChimeraSpit => 64,
            Self::ClimbingGear => 16,
            Self::Clothes => 16,
            Self::Crowbar => 16,
            Self::Crutch => 4,
            Self::Cure => 16,
            Self::Darkroot => 16,
            Self::CrimsonWeed => 16,
            Self::CorpseMandrake => 16,
            Self::Dice => 4,
            Self::DivinationTools => 16,
            Self::DreamSand => 16,
            Self::FieldKitchen => 16,
            Self::FishingTools => 16,
            Self::FlintAndTinder => 4,
            Self::Gemstone => 1024,
            Self::GoldJewel => 64,
            Self::GrapplingHook => 4,
            Self::HealingDraught => 64,
            Self::Incense => 4,
            Self::Ladder => 4,
            Self::Lantern => 4,
            Self::LockboxAndKey => 4,
            Self::LockPicks => 16,
            Self::LookingGlass => 16,
            Self::LuckyCharm => 16,
            Self::MadcapMushroom => 16,
            Self::ManaclesAndKey => 4,
            Self::MedicineBox => 16,
            Self::MetalFile => 4,
            Self::Mirror => 16,
            Self::MusicInstrument => 16,
            Self::Muzzle => 4,
            Self::NavigationTools => 16,
            Self::Net => 16,
            Self::OilLamp => 4,
            Self::Painting => 64,
            Self::Perfume => 16,
            Self::PipeAndTobacco => 4,
            Self::Prosthesis => 16,
            Self::QuillAndInk => 4,
            Self::Ration => 4,
            Self::ReapersSpice => 16,
            Self::Rope => 4,
            Self::RopeLadder => 4,
            Self::RowingBoat => 256,
            Self::Saddle => 16,
            Self::SailingBoat => 1024,
            Self::Scissors => 4,
            Self::ScorpionOil => 16,
            Self::ScrollCase => 4,
            Self::SealingWax => 4,
            Self::SignetRing => 16,
            Self::SilverJewel => 4,
            Self::TrappingTools => 26,
            Self::SurgeryTools => 16,
            Self::Toolbox => 16,
            Self::Torch => 1,
            Self::WarmClothes => 16,
            Self::Water => 1,
            Self::PowerScroll(_) => 64,
            Self::Grimoire => 16,

            // Other items
            Self::SimpleHugeWeapon => 8,
        }
    }

    pub fn bulk(&self) -> i32 {
        // 0 means "light", or half a normal item.
        match self {
            Self::Miscellanea => 0,

            // Armour
            Self::LightArmour => 2,
            Self::HeavyArmour => 4,
            Self::Shield => 2,
            Self::Helmet => 1,

            // Standard weapons
            Self::SimpleHandWeapon => 1,
            Self::SimpleGreatWeapon => 2,
            Self::MartialHandWeapon => 1,
            Self::MartialGreatWeapon => 2,
            Self::MissileHandWeapon => 1,
            Self::MissileGreatWeapon => 2,
            Self::Handgun => 1,
            Self::LongGun => 2,

            // Special weapons
            Self::Garrotte => 1,
            Self::Mancatcher => 2,
            Self::AcidVial => 0,
            Self::FlashPowder => 0,
            Self::AlchemistsFire => 0,
            Self::BlessedWater => 0,
            Self::SmokeBomb => 0,

            // Artillery
            Self::Ballista => 32,
            Self::Catapult => 32,
            Self::Cannon => 32,
            Self::Mortar => 32,

            // Gear
            Self::AlcoholicDrink => 0,
            Self::AnimalSkin => 1,
            Self::Antidote => 0,
            Self::ApothecaryTools => 2,
            Self::BagOfDung => 1,
            Self::BearTrap => 2,
            Self::BeastmanBlood => 0,
            Self::BlackAdderVenom => 0,
            Self::Book => 1,
            Self::Scroll => 0,
            Self::BrushAndPaints => 0,
            Self::ButterflySting => 0,
            Self::Cage => 2,
            Self::Caltrops => 1,
            Self::CampingKit => 2,
            Self::Candle => 0,
            Self::CandlestickRoot => 0,
            Self::Canvas => 2,
            Self::Cards => 0,
            Self::Censer => 2,
            Self::Cart => 32,
            Self::Coach => 64,
            Self::Chain => 1,
            Self::Chalk => 0,
            Self::ChessSet => 1,
            Self::ChimeraSpit => 0,
            Self::ClimbingGear => 2,
            Self::Clothes => 1,
            Self::Crowbar => 1,
            Self::Crutch => 1,
            Self::Cure => 0,
            Self::Darkroot => 0,
            Self::CrimsonWeed => 0,
            Self::CorpseMandrake => 0,
            Self::Dice => 0,
            Self::DivinationTools => 2,
            Self::DreamSand => 0,
            Self::FieldKitchen => 2,
            Self::FishingTools => 2,
            Self::FlintAndTinder => 0,
            Self::Gemstone => 0,
            Self::GoldJewel => 0,
            Self::GrapplingHook => 1,
            Self::HealingDraught => 0,
            Self::Incense => 0,
            Self::Ladder => 4,
            Self::Lantern => 1,
            Self::LockboxAndKey => 1,
            Self::LockPicks => 1,
            Self::LookingGlass => 1,
            Self::LuckyCharm => 0,
            Self::MadcapMushroom => 0,
            Self::ManaclesAndKey => 1,
            Self::MedicineBox => 1,
            Self::MetalFile => 1,
            Self::Mirror => 1,
            Self::MusicInstrument => 1,
            Self::Muzzle => 1,
            Self::NavigationTools => 2,
            Self::Net => 1,
            Self::OilLamp => 0,
            Self::Painting => 2,
            Self::Perfume => 0,
            Self::PipeAndTobacco => 0,
            Self::Prosthesis => 1,
            Self::QuillAndInk => 0,
            Self::Ration => 0,
            Self::ReapersSpice => 0,
            Self::Rope => 1,
            Self::RopeLadder => 2,
            Self::RowingBoat => 32,
            Self::Saddle => 2,
            Self::SailingBoat => 64,
            Self::Scissors => 1,
            Self::ScorpionOil => 0,
            Self::ScrollCase => 1,
            Self::SealingWax => 0,
            Self::SignetRing => 0,
            Self::SilverJewel => 0,
            Self::TrappingTools => 2,
            Self::SurgeryTools => 2,
            Self::Toolbox => 2,
            Self::Torch => 1,
            Self::WarmClothes => 2,
            Self::Water => 0,
            Self::PowerScroll(_) => 0,
            Self::Grimoire => 1,

            // Other items
            Self::SimpleHugeWeapon => 8,
        }
    }

    pub fn keywords(&self) -> Vec<ItemKeyword> {
        match self {
            Self::SimpleHandWeapon => {
                vec![ItemKeyword::Weapon(Weapon::new(Some(Die::D4), Vec::new()))]
            }
            Self::SimpleGreatWeapon => {
                vec![ItemKeyword::Weapon(Weapon::new(Some(Die::D6), Vec::new()))]
            }
            Self::MartialHandWeapon => {
                vec![ItemKeyword::Weapon(Weapon::new(Some(Die::D6), Vec::new()))]
            }
            Self::MartialGreatWeapon => {
                vec![ItemKeyword::Weapon(Weapon::new(Some(Die::D8), Vec::new()))]
            }
            Self::MissileHandWeapon => {
                vec![ItemKeyword::Weapon(Weapon::new(
                    Some(Die::D4),
                    vec![WeaponKeyword::Range(8)],
                ))]
            }
            Self::MissileGreatWeapon => {
                vec![ItemKeyword::Weapon(Weapon::new(
                    Some(Die::D6),
                    vec![WeaponKeyword::Range(8)],
                ))]
            }
            Self::Handgun => {
                vec![ItemKeyword::Weapon(Weapon::new(
                    Some(Die::D6),
                    vec![WeaponKeyword::Range(8), WeaponKeyword::Reload],
                ))]
            }
            Self::LongGun => {
                vec![ItemKeyword::Weapon(Weapon::new(
                    Some(Die::D8),
                    vec![WeaponKeyword::Range(8), WeaponKeyword::Reload],
                ))]
            }

            Self::Ballista => {
                vec![
                    ItemKeyword::Weapon(Weapon::new(
                        Some(Die::D8),
                        vec![
                            WeaponKeyword::Range(16),
                            WeaponKeyword::Blast,
                            WeaponKeyword::Reload,
                        ],
                    )),
                    ItemKeyword::Durability(12, 4),
                ]
            }
            Self::Catapult => {
                vec![
                    ItemKeyword::Weapon(Weapon::new(
                        Some(Die::D8),
                        vec![
                            WeaponKeyword::IndirectRange(32),
                            WeaponKeyword::Blast,
                            WeaponKeyword::Reload,
                        ],
                    )),
                    ItemKeyword::Durability(12, 4),
                ]
            }
            Self::Cannon => {
                vec![
                    ItemKeyword::Weapon(Weapon::new(
                        Some(Die::D10),
                        vec![
                            WeaponKeyword::Range(16),
                            WeaponKeyword::Blast,
                            WeaponKeyword::Reload,
                        ],
                    )),
                    ItemKeyword::Durability(12, 4),
                ]
            }
            Self::Mortar => {
                vec![
                    ItemKeyword::Weapon(Weapon::new(
                        Some(Die::D10),
                        vec![
                            WeaponKeyword::IndirectRange(32),
                            WeaponKeyword::Blast,
                            WeaponKeyword::Reload,
                        ],
                    )),
                    ItemKeyword::Durability(12, 4),
                ]
            }

            Self::AcidVial => {
                vec![
                    ItemKeyword::Consumable,
                    ItemKeyword::Weapon(Weapon::new(Some(Die::D8), vec![WeaponKeyword::Range(2)])),
                ]
            }
            Self::AlchemistsFire => {
                vec![
                    ItemKeyword::Consumable,
                    ItemKeyword::Weapon(Weapon::new(
                        Some(Die::D6),
                        vec![
                            WeaponKeyword::Range(2),
                            WeaponKeyword::Blast,
                            WeaponKeyword::FireDamage,
                        ],
                    )),
                ]
            }
            Self::FlashPowder => {
                vec![
                    ItemKeyword::Consumable,
                    ItemKeyword::Weapon(Weapon::new(
                        None,
                        vec![
                            WeaponKeyword::Range(2),
                            WeaponKeyword::Blast,
                            WeaponKeyword::Stun,
                        ],
                    )),
                ]
            }
            Self::BlessedWater => {
                vec![
                    ItemKeyword::Consumable,
                    ItemKeyword::Weapon(Weapon::new(
                        Some(Die::D10),
                        vec![WeaponKeyword::Range(2), WeaponKeyword::HolyDamage],
                    )),
                ]
            }
            Self::SmokeBomb => {
                vec![ItemKeyword::Consumable]
            }

            Self::Garrotte => {
                vec![ItemKeyword::Weapon(Weapon::new(
                    Some(Die::D8),
                    vec![WeaponKeyword::DirectDamage],
                ))]
            }
            Self::Mancatcher => {
                vec![ItemKeyword::Weapon(Weapon::new(
                    Some(Die::D4),
                    vec![WeaponKeyword::Grab],
                ))]
            }

            Self::LightArmour => {
                vec![ItemKeyword::Armour(1)]
            }
            Self::HeavyArmour => {
                vec![ItemKeyword::Armour(2)]
            }
            Self::Shield => {
                vec![
                    ItemKeyword::Shield,
                    ItemKeyword::Weapon(Weapon::new(Some(Die::D4), Vec::new())),
                ]
            }

            Self::Cart => vec![ItemKeyword::Durability(12, 4), ItemKeyword::Vehicle(4)],
            Self::Coach => vec![ItemKeyword::Durability(16, 4), ItemKeyword::Vehicle(8)],
            Self::RowingBoat => vec![ItemKeyword::Durability(12, 4), ItemKeyword::Vehicle(4)],
            Self::SailingBoat => vec![ItemKeyword::Durability(16, 4), ItemKeyword::Vehicle(8)],

            Self::AlcoholicDrink => vec![ItemKeyword::Consumable],
            Self::Antidote => vec![ItemKeyword::Consumable],
            Self::BeastmanBlood => vec![ItemKeyword::Consumable],
            Self::BlackAdderVenom => vec![ItemKeyword::Consumable],
            Self::BrushAndPaints => vec![ItemKeyword::Consumable],
            Self::ButterflySting => vec![ItemKeyword::Consumable],
            Self::Candle => vec![ItemKeyword::Consumable],
            Self::CandlestickRoot => vec![ItemKeyword::Consumable],
            Self::Chalk => vec![ItemKeyword::Consumable],
            Self::ChimeraSpit => vec![ItemKeyword::Consumable],
            Self::Cure => vec![ItemKeyword::Consumable],
            Self::Darkroot => vec![ItemKeyword::Consumable],
            Self::CrimsonWeed => vec![ItemKeyword::Consumable],
            Self::CorpseMandrake => vec![ItemKeyword::Consumable],
            Self::DreamSand => vec![ItemKeyword::Consumable],
            Self::HealingDraught => vec![ItemKeyword::Consumable],
            Self::Incense => vec![ItemKeyword::Consumable],
            Self::MadcapMushroom => vec![ItemKeyword::Consumable],
            Self::MedicineBox => vec![ItemKeyword::Consumable],
            Self::OilLamp => vec![ItemKeyword::Consumable],
            Self::Perfume => vec![ItemKeyword::Consumable],
            Self::PipeAndTobacco => vec![ItemKeyword::Consumable],
            Self::QuillAndInk => vec![ItemKeyword::Consumable],
            Self::Ration => vec![ItemKeyword::Consumable],
            Self::ReapersSpice => vec![ItemKeyword::Consumable],
            Self::ScorpionOil => vec![ItemKeyword::Consumable],
            Self::Torch => vec![ItemKeyword::Consumable],
            Self::Water => vec![ItemKeyword::Consumable],
            Self::SealingWax => vec![ItemKeyword::Consumable],

            // Other items
            Self::SimpleHugeWeapon => {
                vec![ItemKeyword::Weapon(Weapon::new(Some(Die::D6), Vec::new()))]
            }

            _ => Vec::new(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::SimpleHandWeapon => String::from(
                "Short blades, blunt objects, and one-handed sharp or heavy tools: bullwhip, \
                 cleaver, club, cudgel, dagger, knife, knuckledusters, hammer, hatchet, long \
                 knife, parrying dagger, scalpel, sickle, sturdy stick, etc.",
            ),
            Self::SimpleGreatWeapon => String::from(
                "Two-handed sharp or heavy tools: big stick, boat hook, hoe, large club, \
                 linstock, pickaxe, pitchfork, quarterstaff, scythe, sledgehammer, shovel, spade, \
                 staff, woodcutting axe, etc.",
            ),
            Self::MartialHandWeapon => String::from(
                "One-handed military grade melee weapons: arming sword, battle axe, flail, mace, \
                 morningstar, spear, thrusting sword, warhammer, etc.",
            ),
            Self::MartialGreatWeapon => String::from(
                "Two-handed military grade melee weapons: glaive, halberd, heavy battle axe, \
                 heavy flail, lucerne hammer, pike, voulge, zweihänder, etc.",
            ),
            Self::MissileHandWeapon => String::from(
                "One-handed primitive ranged or throwing weapons: blowgun, darts, hand crossbow, \
                 harpoons, javelins, sling, throwing knives, throwing stars, etc.",
            ),
            Self::MissileGreatWeapon => {
                String::from("Two-handed primitive ranged weapons: bow, crossbow, etc.")
            }
            Self::Handgun => {
                String::from("One-handed gunpowder weapon: matchlock pistol, dragon, etc.")
            }
            Self::LongGun => {
                String::from("Two-handed gunpowder weapon: matchlock arquebus, blunderbuss, etc.")
            }

            Self::BlessedWater => format!(""),
            Self::SmokeBomb => String::from(
                "Throw it at range 2. All zones within range 1 of the targeted one are covered in \
                 thick smoke, blocking visibility until the end of the stretch",
            ),

            Self::Garrotte => String::from(
                "Can only target characters who are unaware of the attack or whom you are \
                 currently grabbing.",
            ),

            Self::LightArmour => String::from(
                "Banded mail, chain mail, gambeson, hardened leather, thick furs, etc.",
            ),
            Self::HeavyArmour => String::from("Full plate, chain hauberk, etc."),
            Self::Shield => String::from(""),
            Self::Helmet => String::from(
                "The threshold to suffer critical damage is increased by 1, that is you suffer \
                 critical damage when you lose half your health plus 1. You still always suffer \
                 critical damage if you are reduced to 0 health, and direct damage ignores this \
                 effect.",
            ),

            Self::Cart => format!(""),
            Self::Coach => format!(""),
            Self::RowingBoat => format!(
                "Must be rowed. Moves by 1 area per time unit per rowing character, maximum 4."
            ),
            Self::SailingBoat => format!(
                "Must be operated by a character with the ~{}~ skill or rowed. Moves between 1 \
                 and 6 areas per time unit with the wind, or by half area per time unit per \
                 rowing character if rowed, maximum 4.",
                Skill::Boatmanship
            ),

            Self::AlcoholicDrink => format!(
                "Recover 1 health, but not if you are ~{}~. Pass a STR save or become ~{}~ until \
                 the end of the stretch. Automatically fail if you drink 2 within a stretch.",
                Condition::Dying,
                Condition::Confused,
            ),
            Self::AnimalSkin => format!(
                "Skin of a {} animal. Multiply cost by 4 and bulk by 2 for each size above {}.",
                SizeCategory::Small,
                SizeCategory::Small,
            ),
            Self::Antidote => {
                format!("Stops the effects of poison. 1:4 chance it doesn't work.")
            }
            Self::ApothecaryTools => format!(
                "Includes mortar, pestle, scales, glassware, etc. Used in combination with the \
                 ~{}~ skill.",
                Skill::Apothecary
            ),
            Self::BagOfDung => format!("Stinky and disgusting"),
            Self::BearTrap => format!(
                "Placing it or picking it up is a main action, concealing it takes a stretch. \
                 Characters stepping on it suffer d6 damage and are ~{}~.",
                Condition::Entangled
            ),
            Self::BeastmanBlood => format!("Ingested corrupting poison."),
            Self::BlackAdderVenom => format!("Bloodstream damaging poison."),
            Self::Book => format!("The price is for a blank book."),
            Self::Scroll => format!("The price is for a blank scroll."),
            Self::BrushAndPaints => format!("Enough to paint a ~{}~.", ItemKind::Canvas),
            Self::ButterflySting => format!("Bloodstream soporific poison."),
            Self::Cage => format!("Can contain a small character."),
            Self::Caltrops => format!(
                "As a main action, can be spread in a nearby zone, which counts as difficult \
                 ground. Picking them up takes a stretch."
            ),
            Self::CampingKit => format!(
                "Includes a bedroll, blankets, and tent space for one person. Necessary to sleep \
                 in the wilderness."
            ),
            Self::Candle => format!("Illuminates a zone until the end of the stretch."),
            Self::CandlestickRoot => format!("Ingested paralysing poison."),
            Self::Canvas => format!("About 1 metre by 1 metre."),
            Self::Cards => format!("WIT contest to win."),
            Self::Censer => format!("Can be used to burn ~{}~.", ItemKind::Incense),
            Self::Chain => format!("2 metre."),
            Self::Chalk => format!("Enough to write a few markings."),
            Self::ChessSet => format!("WIT contest to win."),
            Self::ChimeraSpit => format!("Bloodstream lethal poison."),
            Self::ClimbingGear => format!(
                "Includes pitons, nails, climbing pickaxes, etc. Can be used to scale sheer \
                 surfaces, but it's challenging."
            ),
            Self::Clothes => format!(
                "All you need to cover yourself! Include belts, pouches, bags, and other \
                 containers to hold reasonably sized items."
            ),
            Self::Crowbar => format!(
                "Spend a stretch and pass a STR save to open a door, chest, etc. Noisy. Can be \
                 used as a ~{}~.",
                ItemKind::SimpleHandWeapon
            ),
            Self::Crutch => format!("~{}~ characters can use it to walk.", Condition::Hobbled),
            Self::Cure => {
                format!("Take it before a recovery roll to pass. 1:4 chance it doesn't work.")
            }
            Self::Darkroot => format!(
                "Pass a STR save or be ~{}~ until the end of the watch, as you are overwhelmed by \
                 pleasant hallucinations. Automatically fail if you take 2 within a stretch. \
                 After the effect wears off, pass a WIT save or become ~{}~.",
                Condition::Confused,
                Trait::Addicted
            ),
            Self::CrimsonWeed => format!(
                "Pass a STR save or temporarily increase all abilities by 2. Automatically fail \
                 if you take 2 within a stretch. At the end of the watch, temporarily reduce all \
                 abilities by 4, and pass a WIT save or become ~{}~.",
                Trait::Addicted
            ),
            Self::CorpseMandrake => format!(
                "Pass a STR save or become numb to pain until the end of the watch: you can't be \
                 ~{}~ by critical damage. Automatically fail if you take 2 within a stretch. \
                 After the effect wears off, suffer d4 direct damage, and pass a WIT save or \
                 become ~{}~.",
                Condition::Incapacitated,
                Trait::Addicted
            ),
            Self::Dice => format!("Luck-based game."),
            Self::DivinationTools => format!(
                "Include a crystal ball, tarot cards, divination bones, etc. Used in combination \
                 with the ~{}~ skill.",
                Skill::Divination
            ),
            Self::DreamSand => format!("Ingested soporific poison."),
            Self::FieldKitchen => format!(
                "Includes pots, pans, kitchen utensils, etc. Food you gather by foraging is \
                 doubled and doesn't go bad."
            ),
            Self::FishingTools => format!(
                "Obtain a ~{}~ on a ~fish~ result while foraging.",
                ItemKind::Ration
            ),
            Self::FlintAndTinder => format!("Can be used to start a fire"),
            Self::Gemstone => format!("Extremely shiny and valuable."),
            Self::GoldJewel => format!("Very shiny and valuable."),
            Self::GrapplingHook => format!("Use it to throw ropes and such."),
            Self::HealingDraught => format!("Recover health equal to half STR."),
            Self::Incense => format!(
                "Burns for a stretch. ~{}s~ are disturbed by it: they must pass a STR save to \
                 enter a zone where incense is burning.",
                CharacterCategory::Demon
            ),
            Self::Ladder => format!("Climbing it is trivial."),
            Self::Lantern => format!(
                "Holds a ~{}~ or an ~{}~, protecting it from wind and water. Can screen the light.",
                ItemKind::Candle,
                ItemKind::OilLamp
            ),
            Self::LockboxAndKey => format!("Can contain a total bulk of 1."),
            Self::LockPicks => format!("Spend a stretch and pass a WIT save to open a lock."),
            Self::LookingGlass => format!("See distant details."),
            Self::LuckyCharm => format!(
                "When you spend an omen, there is 1:4 chance it isn't actually spent. If combined \
                 with the ~{}~ skill, You have in total a 1:2 chance of not losing the omen.",
                Skill::Luck
            ),
            Self::MadcapMushroom => format!(
                "Pass a STR save or become ~{}~ until the end of the stretch. Automatically fail \
                 if you take 2 within a stretch. After the effect wears off, pass a WIT save or \
                 become ~{}~.",
                Condition::Frenzied,
                Trait::Addicted
            ),
            Self::ManaclesAndKey => format!("Used to bind someone's hands."),
            Self::MedicineBox => format!(
                "Spend a stretch to let a nearby character recover d8 health, but no more than \
                 half their maximum."
            ),
            Self::MetalFile => format!("Can cut through metal, with enough time."),
            Self::Mirror => format!("A simple hand mirror."),
            Self::MusicInstrument => format!("Drums, fiddle, flute, Lyre, Lute, etc."),
            Self::Muzzle => format!("To muzzle a dog or similarly sized animal."),
            Self::NavigationTools => format!(
                "Include maps, compass, etc. Pass a WIT save to ignore the movement penalty when \
                 travelling between sectors or regions without following a path."
            ),
            Self::Net => format!("Throw it on an enemy: they are ~{}~.", Condition::Entangled,),
            Self::OilLamp => {
                format!("Illuminates the zones within range 1 until the end of the stretch.")
            }
            Self::Painting => format!("A nice painting."),
            Self::Perfume => format!("An exquisite perfume"),
            Self::PipeAndTobacco => format!(
                "Smoke it after failing a save in a situation requiring thinking to repeat the \
                 save once. Then pass a WIT save or become ~{}~ to it.",
                Trait::Addicted
            ),
            Self::Prosthesis => format!(
                "Replaces a lost limb. If it replaces a lost leg, you can walk without a crutch."
            ),
            Self::QuillAndInk => format!("Enough to write a scroll."),
            Self::Ration => format!("Enough food for a day rest."),
            Self::ReapersSpice => format!("Ingested damaging poison."),
            Self::Rope => format!("4 metre."),
            Self::RopeLadder => format!("2 metre. Trivial to climb."),
            Self::Saddle => format!("Required to ride a mount. Includes saddle bags."),
            Self::Scissors => format!("Used to cut precisely."),
            Self::ScorpionOil => format!("Bloodstream paralysing poison."),
            Self::ScrollCase => {
                format!("Can hold 2 scrolls, protecting them from water and damage.")
            }
            Self::SealingWax => format!(
                "Used to seal letters in combination with a ~{}~.",
                ItemKind::SignetRing
            ),
            Self::SignetRing => format!(
                "Proof of identity, used to stamp ~{}~.",
                ItemKind::SealingWax
            ),
            Self::SilverJewel => format!("Shiny and valuable."),
            Self::TrappingTools => format!(
                "Obtain a ~{}~ on a ~small game~ result while foraging.",
                ItemKind::Ration
            ),
            Self::SurgeryTools => format!(
                "Include hacksaws, scalpels, knifes, stitches, etc. Performing a surgery takes a \
                 stretch and requires passing a WIT save. On a failure the patient suffers d8 \
                 direct damage."
            ),
            Self::Toolbox => format!(
                "Includes hammer, saws, nails, etc. Spend a watch and pass a WIT save to repair a \
                 damaged item, but on a fail it is destroyed. If the item has the ~durability~ \
                 keyword, repair or inflict 2 damage instead on a pass or fail.",
            ),
            Self::Torch => format!(
                "Illuminates the zones within range 1 until the end of the stretch. Can be used \
                 to attack, inflicting d4 fire damage, but follows the improvised weapon rules."
            ),
            Self::WarmClothes => format!("Cold damage is impaired, heat damage is enhanced."),
            Self::Water => format!("Enough for a day rest."),
            Self::PowerScroll(_) => {
                format!(
                    "A scroll recording a sacred or sorcerous power. Trading them is illegal. The \
                     knowledge of how to create them has been lost."
                )
            }
            Self::Grimoire => format!(
                "Can hold up to 4 ~{}s~, removing the need to swamp them out to invoke their \
                 power.",
                ItemKind::PowerScroll(PowerContent::Undefined)
            ),

            _ => String::new(),
        }
    }
}

static_gen_fn!(armour, gen_armour, ItemKind, ItemKind, {
    vec![
        ItemKind::Helmet,
        ItemKind::Shield,
        ItemKind::LightArmour,
        ItemKind::HeavyArmour,
    ]
});

static_gen_fn!(weapons, gen_weapons, ItemKind, ItemKind, {
    vec![
        ItemKind::SimpleHandWeapon,
        ItemKind::SimpleGreatWeapon,
        ItemKind::MartialHandWeapon,
        ItemKind::MartialGreatWeapon,
        ItemKind::MissileHandWeapon,
        ItemKind::MissileGreatWeapon,
        ItemKind::Handgun,
        ItemKind::LongGun,
        ItemKind::Garrotte,
        ItemKind::Mancatcher,
        ItemKind::AcidVial,
        ItemKind::AlchemistsFire,
        ItemKind::BlessedWater,
        ItemKind::FlashPowder,
    ]
});

static_gen_fn!(artillery, gen_artillery, ItemKind, ItemKind, {
    vec![
        ItemKind::Ballista,
        ItemKind::Catapult,
        ItemKind::Cannon,
        ItemKind::Mortar,
    ]
});

static_gen_fn!(vehicles, gen_vehicles, ItemKind, ItemKind, {
    vec![
        ItemKind::Cart,
        ItemKind::Coach,
        ItemKind::RowingBoat,
        ItemKind::SailingBoat,
    ]
});

static_gen_fn!(gear, gen_gear, ItemKind, ItemKind, {
    vec![
        Self::AlcoholicDrink,
        Self::AnimalSkin,
        Self::Antidote,
        Self::ApothecaryTools,
        Self::BagOfDung,
        Self::BearTrap,
        Self::BeastmanBlood,
        Self::BlackAdderVenom,
        Self::Book,
        Self::BrushAndPaints,
        Self::ButterflySting,
        Self::Cage,
        Self::Caltrops,
        Self::CampingKit,
        Self::Candle,
        Self::CandlestickRoot,
        Self::Canvas,
        Self::Cards,
        Self::Censer,
        Self::Chain,
        Self::Chalk,
        Self::ChessSet,
        Self::ChimeraSpit,
        Self::ClimbingGear,
        Self::Clothes,
        Self::CorpseMandrake,
        Self::CrimsonWeed,
        Self::Crowbar,
        Self::Crutch,
        Self::Cure,
        Self::Darkroot,
        Self::Dice,
        Self::DivinationTools,
        Self::DreamSand,
        Self::FieldKitchen,
        Self::FishingTools,
        Self::FlintAndTinder,
        Self::Gemstone,
        Self::GoldJewel,
        Self::GrapplingHook,
        Self::Grimoire,
        Self::HealingDraught,
        Self::Incense,
        Self::Ladder,
        Self::Lantern,
        Self::LockboxAndKey,
        Self::LockPicks,
        Self::LookingGlass,
        Self::LuckyCharm,
        Self::MadcapMushroom,
        Self::ManaclesAndKey,
        Self::MedicineBox,
        Self::MetalFile,
        Self::Mirror,
        Self::MusicInstrument,
        Self::Muzzle,
        Self::NavigationTools,
        Self::Net,
        Self::OilLamp,
        Self::Painting,
        Self::Perfume,
        Self::PipeAndTobacco,
        Self::PowerScroll(PowerContent::Undefined),
        Self::Prosthesis,
        Self::QuillAndInk,
        Self::Ration,
        Self::ReapersSpice,
        Self::Rope,
        Self::RopeLadder,
        Self::Saddle,
        Self::Scissors,
        Self::ScorpionOil,
        Self::Scroll,
        Self::ScrollCase,
        Self::SealingWax,
        Self::SignetRing,
        Self::SilverJewel,
        Self::SmokeBomb,
        Self::SurgeryTools,
        Self::Toolbox,
        Self::Torch,
        Self::TrappingTools,
        Self::WarmClothes,
        Self::Water,
    ]
});
