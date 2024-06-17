use character::*;

pub fn bloodletter() -> CharacterProfile {
    CharacterProfile {
        name: String::from("Agruwalsongor the Bloodletter"),
        cost: None,
        abilities: Abilities::new(16, 4, 8),
        mana: 0,
        category: CharacterCategory::Demon,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::HumanIntelligence,
        skills: vec![Skill::SkilledStrike, Skill::Brawling],
        traits: vec![Trait::Frightening],
        assets: vec![Asset::Item(Item::new(
            Some("demonic greatsword"),
            ItemKind::MartialGreatWeapon,
            None,
        ))],
        natural_armour: None,
        natural_weapons: Vec::new(),
        description: Some(String::from(
            "A large horned man brandishing a giant sword, his skin constantly exuding fresh \
             blood. Collects the skulls of slain mortals.",
        )),
        special_rules: vec![(
            String::from("frenzy"),
            format!(
                "Constantly ~{}~, wants to kill all creatures with no exception.",
                Condition::Frenzied
            ),
        )],
    }
}

pub fn corruptor() -> CharacterProfile {
    CharacterProfile {
        name: String::from("Vlashmandarka the Corruptor"),
        cost: None,
        abilities: Abilities::new(8, 8, 12),
        mana: 0,
        category: CharacterCategory::Demon,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::HumanIntelligence,
        skills: vec![Skill::Charm],
        traits: Vec::new(),
        assets: Vec::new(),
        natural_armour: None,
        natural_weapons: vec![(
            "barbed tongue-whip",
            Weapon::new(Some(Die::D4), vec![WeaponKeyword::Grab]),
        )],
        description: Some(String::from(
            "Takes the form of the most attractive creature which the observer can possibly \
             imagine. Wants to corrupt the pure.",
        )),
        special_rules: vec![(
            String::from("powers"),
            format!("Can invoke ~{}~ once per watch at level 4.", Power::Bewitch),
        )],
    }
}

pub fn horror() -> CharacterProfile {
    CharacterProfile {
        name: String::from("Pancratius Pinkflame the Horror"),
        cost: None,
        abilities: Abilities::new(8, 12, 8),
        mana: 0,
        category: CharacterCategory::Demon,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::HumanIntelligence,
        skills: vec![Skill::FastDodge],
        traits: vec![Trait::Frightening],
        assets: Vec::new(),
        natural_armour: None,
        natural_weapons: vec![
            ("claws", Weapon::new(Some(Die::D6), vec![])),
            (
                "eldritch fire",
                Weapon::new(Some(Die::D6), vec![WeaponKeyword::Range(8)]),
            ),
        ],
        description: Some(String::from(
            "A mess of flesh and bones, shifting in shape and bright colours. Sees mortals as \
             clay to be mould.",
        )),
        special_rules: vec![
            (
                String::from("corrupting touch"),
                String::from(
                    "Anyone touched by it suffers 2 corruption. This includes targets of melee \
                     attacks, unless they are successfully dodge.",
                ),
            ),
            (
                String::from("split"),
                String::from(
                    "When it suffers critical damage it splits into two copies of itself, each \
                     with health equal to the remaining amount and STR to match.",
                ),
            ),
        ],
    }
}

pub fn plague_brewer() -> CharacterProfile {
    CharacterProfile {
        name: String::from("Kreftelgor the Plague Brewer"),
        cost: None,
        abilities: Abilities::new(8, 4, 8),
        mana: 0,
        category: CharacterCategory::Demon,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::HumanIntelligence,
        skills: vec![Skill::Apothecary],
        traits: vec![Trait::Frightening],
        assets: vec![
            Asset::Item(Item::new(
                None,
                ItemKind::Clothes,
                Some("filthy hooded cloak"),
            )),
            Asset::Item(Item::new(
                Some("rusty dagger"),
                ItemKind::SimpleHandWeapon,
                None,
            )),
        ],
        natural_armour: Some(("bloated flesh", 1)),
        natural_weapons: Vec::new(),
        description: Some(String::from(
            "A skull-faced man with horribly bloated skin, covered in pustules and boils. Creates \
             new diseases and experiments on humans.",
        )),
        special_rules: vec![(
            String::from("disease vials"),
            String::from(
                "Carries d4 vials holding various diseases. They can be thrown at range 2 or be \
                 used to coat weapons or taint items and surfaces.",
            ),
        )],
    }
}
