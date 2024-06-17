use character::*;

pub fn wyrm() -> CharacterProfile {
    CharacterProfile {
        name: String::from("wyrm"),
        abilities: Abilities::new(16, 4, 12),
        category: CharacterCategory::Creature,
        size: SizeCategory::Large,
        intelligence: IntelligenceCategory::AnimalIntelligence,
        traits: vec![Trait::Frightening, Trait::LandingFlyer],
        natural_armour: Some(("scales", 2)),
        natural_weapons: vec![
            (
                "bite, talons, & tail",
                Weapon::new(Some(Die::D8), Vec::new()),
            ),
            (
                "fire breath",
                Weapon::new(
                    Some(Die::D6),
                    vec![
                        WeaponKeyword::Range(2),
                        WeaponKeyword::Blast,
                        WeaponKeyword::FireDamage,
                        WeaponKeyword::UsageLimit("once per stretch"),
                    ],
                ),
            ),
        ],
        description: Some(String::from(
            "A gigantic lizard-like winged creature, capable of breathing fire.",
        )),
        ..Default::default()
    }
}

pub fn troll() -> CharacterProfile {
    CharacterProfile {
        name: String::from("troll"),
        abilities: Abilities::new(12, 4, 4),
        category: CharacterCategory::Creature,
        size: SizeCategory::Large,
        intelligence: IntelligenceCategory::HumanIntelligence,
        traits: vec![Trait::Frightening, Trait::Regeneration],
        natural_armour: Some(("thick skin", 1)),
        natural_weapons: Vec::new(),
        assets: vec![Asset::from(Item::new(
            Some("huge club"),
            ItemKind::SimpleHugeWeapon,
            Some("an extremely large club, impossible to use for a medium-sized character"),
        ))],
        description: Some(String::from(
            "A large, brutish, humanoid monster, capable of quick regeneration. Lives in swamps \
             and feasts on rotten meat.",
        )),
        ..Default::default()
    }
}
