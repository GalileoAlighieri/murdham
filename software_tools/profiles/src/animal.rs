use character::*;

fn calculate_ability_score(score: i8) -> i32 {
    if score >= 16 {
        4
    } else if score >= 12 {
        3
    } else if score >= 8 {
        2
    } else if score >= 4 {
        1
    } else {
        0
    }
}

fn calculate_animal_cost(c: &mut CharacterProfile) {
    let mut score = 0;
    score += calculate_ability_score(c.abilities.str);
    score += calculate_ability_score(c.abilities.agi);
    // WIT intentionally ignored.

    match c.size {
        SizeCategory::Tiny => score -= 4,
        SizeCategory::Small => score -= 2,
        SizeCategory::MediumSized => score += 0,
        SizeCategory::Large => score += 2,
        SizeCategory::Massive => score += 4,
    }

    if let Some((_, natural_armour)) = c.natural_armour {
        score += natural_armour as i32;
    }

    for (_, weapon) in c.natural_weapons.iter() {
        if let Some(die) = weapon.damage {
            match die {
                Die::D4 => score += 0,
                Die::D6 => score += 1,
                Die::D8 => score += 2,
                Die::D10 => score += 3,
                Die::D12 => score += 4,
            }
        }
    }

    score += c.traits.len() as i32;

    let cost = if score < 0 { 1 } else { 2i32.pow(score as u32) };

    c.cost = Some(cost);
}

pub fn animal() -> CharacterProfile {
    CharacterProfile {
        name: String::from("animal"),
        cost: None,
        abilities: Abilities::new(8, 8, 8),
        mana: 0,
        category: CharacterCategory::Creature,
        size: SizeCategory::MediumSized,
        intelligence: IntelligenceCategory::AnimalIntelligence,
        skills: Vec::new(),
        traits: Vec::new(),
        assets: Vec::new(),
        natural_armour: None,
        natural_weapons: Vec::new(),
        description: None,
        special_rules: Vec::new(),
    }
}

pub fn camel() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("camel");
    profile.abilities.str = 12;
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::Sturdy);
    profile
        .natural_weapons
        .push(("kick", Weapon::new(Some(Die::D4), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn cat() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("cat");
    profile.abilities.str = 4;
    profile.abilities.agi = 12;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("bite & claws", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn cattle() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("cattle");
    profile.abilities.str = 16;
    profile.abilities.agi = 4;
    profile.traits.push(Trait::Sturdy);
    profile
        .natural_weapons
        .push(("hooves & horns", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn chicken() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("chicken");
    profile.abilities.str = 4;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("beak", Weapon::new(Some(Die::D4), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn small_dog() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("small dog");
    profile.abilities.str = 4;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("bite", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn medium_dog() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("medium dog");
    profile.abilities.str = 4;
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("bite", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn large_dog() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("large dog");
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("bite", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn donkey() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("donkey");
    profile.abilities.agi = 4;
    profile.traits.push(Trait::Sturdy);
    profile
        .natural_weapons
        .push(("hooves", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn duck() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("duck");
    profile.abilities.str = 4;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::SwoopingFlyer);
    profile
        .natural_weapons
        .push(("beak", Weapon::new(Some(Die::D4), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn goat() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("goat");
    profile.abilities.str = 4;
    profile.abilities.agi = 4;
    profile
        .natural_weapons
        .push(("horns", Weapon::new(Some(Die::D4), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn light_horse() -> CharacterProfile {
    let mut profile = animal();
    profile.abilities.str = 12;
    profile.name = String::from("light horse");
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::Sturdy);
    profile
        .natural_weapons
        .push(("hooves", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn heavy_horse() -> CharacterProfile {
    let mut profile = animal();
    profile.abilities.str = 16;
    profile.name = String::from("heavy horse");
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::Sturdy);
    profile
        .natural_weapons
        .push(("hooves", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn mule() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("mule");
    profile.abilities.str = 12;
    profile.abilities.agi = 4;
    profile.traits.push(Trait::Sturdy);
    profile
        .natural_weapons
        .push(("hooves", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn pig() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("pig");
    profile.abilities.str = 4;
    profile.abilities.agi = 4;
    profile
        .natural_weapons
        .push(("bite", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn pigeon() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("pigeon");
    profile.abilities.str = 2;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::SwoopingFlyer);
    profile
        .natural_weapons
        .push(("beak", Weapon::new(Some(Die::D4), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn pony() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("pony horse");
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::Sturdy);
    profile
        .natural_weapons
        .push(("hooves", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn rabbit() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("rabbit");
    profile.abilities.str = 4;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("bite", Weapon::new(Some(Die::D4), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn sheep() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("sheep");
    profile.abilities.str = 4;
    profile.abilities.agi = 4;
    profile
        .natural_weapons
        .push(("hooves", Weapon::new(Some(Die::D4), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn alligator() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("alligator");
    profile.abilities.str = 12;
    profile.abilities.agi = 4;
    profile.natural_armour = Some(("thick scales", 1));
    profile
        .natural_weapons
        .push(("bite", Weapon::new(Some(Die::D8), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn bat() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("bat");
    profile.abilities.str = 2;
    profile.abilities.agi = 12;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::SwoopingFlyer);
    profile
        .natural_weapons
        .push(("bite", Weapon::new(Some(Die::D4), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn bear() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("bear");
    profile.abilities.str = 16;
    profile.abilities.agi = 4;
    profile.traits.push(Trait::Sturdy);
    profile
        .natural_weapons
        .push(("bite & claws", Weapon::new(Some(Die::D8), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn boar() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("boar");
    profile.abilities.agi = 4;
    profile.traits.push(Trait::Sturdy);
    profile
        .natural_weapons
        .push(("tusks", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn deer() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("deer");
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("antlers", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn eagle() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("eagle");
    profile.abilities.str = 4;
    profile.abilities.agi = 12;
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::SwoopingFlyer);
    profile
        .natural_weapons
        .push(("beak & talons", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn elephant() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("elephant");
    profile.abilities.str = 12;
    profile.abilities.agi = 4;
    profile.size = SizeCategory::Large;
    profile.traits.push(Trait::Sturdy);
    profile.natural_armour = Some(("thick hide", 1));
    profile
        .natural_weapons
        .push(("tusks", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn hawk() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("hawk");
    profile.abilities.str = 4;
    profile.abilities.agi = 12;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::SwoopingFlyer);
    profile
        .natural_weapons
        .push(("beak & talons", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn lion() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("lion");
    profile.abilities.str = 12;
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::Sturdy);
    profile
        .natural_weapons
        .push(("bite & claws", Weapon::new(Some(Die::D8), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn lynx() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("lynx");
    profile.abilities.str = 4;
    profile.abilities.agi = 12;
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("bite & claws", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn mouse() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("mouse");
    profile.abilities.str = 2;
    profile.abilities.agi = 12;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("bite", Weapon::new(Some(Die::D4), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn owl() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("owl");
    profile.abilities.str = 4;
    profile.abilities.agi = 12;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::SwoopingFlyer);
    profile
        .natural_weapons
        .push(("beak & talons", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn rat() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("rat");
    profile.abilities.str = 4;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("bite", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn raven() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("raven");
    profile.abilities.str = 4;
    profile.abilities.agi = 12;
    profile.size = SizeCategory::Small;
    profile.traits.push(Trait::Fast);
    profile.traits.push(Trait::SwoopingFlyer);
    profile
        .natural_weapons
        .push(("beak & talons", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn tiger() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("tiger");
    profile.abilities.str = 12;
    profile.abilities.agi = 12;
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("bite & claws", Weapon::new(Some(Die::D8), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}

pub fn viper() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("viper");
    profile.abilities.str = 2;
    profile.size = SizeCategory::Small;
    profile.natural_weapons.push((
        "bite",
        Weapon::new(Some(Die::D4), vec![WeaponKeyword::Poison("damaging")]),
    ));
    calculate_animal_cost(&mut profile);
    // Account for venom.
    if let Some(cost) = &mut profile.cost {
        *cost *= 2;
    }
    profile
}

pub fn wolf() -> CharacterProfile {
    let mut profile = animal();
    profile.name = String::from("wolf");
    profile.abilities.str = 8;
    profile.traits.push(Trait::Fast);
    profile
        .natural_weapons
        .push(("bite & claws", Weapon::new(Some(Die::D6), Vec::new())));
    calculate_animal_cost(&mut profile);
    profile
}
