use crate::raven;
use character::*;

pub fn bone_raven() -> CharacterProfile {
    let mut profile = raven();
    profile.name = String::from("bone raven");
    profile.cost = None;
    profile.abilities.agi /= 2;
    profile.abilities.wit = 2;
    profile.category = CharacterCategory::Undead;
    profile.intelligence = IntelligenceCategory::Mindless;
    profile.skills = Vec::new();
    profile.traits.push(Trait::Frightening);
    profile.description = Some(String::from(
        "a dead raven, empty-eyed, bones carrying rotting flesh.",
    ));
    profile
}

pub fn shambling_corpse() -> CharacterProfile {
    CharacterProfile {
        name: String::from("shambling corpse"),
        cost: None,
        abilities: Abilities::new(8, 4, 2),
        mana: 0,
        category: CharacterCategory::Undead,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::Mindless,
        skills: Vec::new(),
        traits: vec![Trait::Frightening],
        assets: Vec::new(),
        natural_armour: None,
        natural_weapons: Vec::new(),
        description: Some(String::from(
            "a corpse reanimated by dark magic, left to rot.",
        )),
        special_rules: Vec::new(),
    }
}

pub fn skeletal_warrior() -> CharacterProfile {
    CharacterProfile {
        name: String::from("skeletal warrior"),
        cost: None,
        abilities: Abilities::new(8, 4, 2),
        mana: 0,
        category: CharacterCategory::Undead,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::Mindless,
        skills: Vec::new(),
        traits: vec![Trait::Frightening],
        assets: vec![
            Asset::from(Item::from(ItemKind::MartialHandWeapon)),
            Asset::from(Item::from(ItemKind::Shield)),
        ],
        natural_armour: None,
        natural_weapons: Vec::new(),
        description: Some(String::from(
            "an ancient body awakened to serve as an immortal warrior.",
        )),
        special_rules: Vec::new(),
    }
}
