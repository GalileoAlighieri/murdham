use character::*;

pub fn human() -> CharacterProfile {
    CharacterProfile {
        name: String::from("human"),
        abilities: Abilities::new(8, 8, 8),
        category: CharacterCategory::Creature,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::HumanIntelligence,
        description: Some(String::from("The average inhabitant of the Kingdom.")),
        ..Default::default()
    }
}

pub fn dwarf() -> CharacterProfile {
    CharacterProfile {
        name: String::from("dwarf"),
        abilities: Abilities::new(10, 6, 8),
        category: CharacterCategory::Creature,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::HumanIntelligence,
        description: Some(String::from(
            "Short and stocky, clumsy but strong. They live reclusively in underground cities and \
             are known for their craftsmanship.",
        )),
        ..Default::default()
    }
}

pub fn halfling() -> CharacterProfile {
    CharacterProfile {
        name: String::from("halfling"),
        abilities: Abilities::new(6, 10, 8),
        category: CharacterCategory::Creature,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::HumanIntelligence,
        description: Some(String::from(
            "Small and frail, but extremely nimble. Halfling communities are small and live \
             simply, trying to keep distance from the chaotic outside world.",
        )),
        ..Default::default()
    }
}

pub fn elf() -> CharacterProfile {
    CharacterProfile {
        name: String::from("elf"),
        abilities: Abilities::new(8, 10, 10),
        category: CharacterCategory::Creature,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::HumanIntelligence,
        description: Some(String::from(
            "Dexterous and wise, they look like tall, beautiful, flawless humans. They are known \
             for their pride and arrogance, which in turn makes them subject of dislike from the \
             other kins.",
        )),
        ..Default::default()
    }
}

pub fn ogre() -> CharacterProfile {
    CharacterProfile {
        name: String::from("ogre"),
        abilities: Abilities::new(12, 6, 6),
        category: CharacterCategory::Creature,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::HumanIntelligence,
        description: Some(String::from(
            "Giant mountains of muscle. Not the brightest nor the most agile, but their brawn \
             serves them well in many occasions.",
        )),
        ..Default::default()
    }
}
