use asciidoc::AsciiDoc;
use character::*;
use clap::Parser;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
use std::{fs::File, io::Write, path::PathBuf};

fn random_element<T: Clone>(rng: &mut ThreadRng, v: &[T]) -> T {
    v[rng.gen_range(0..v.len())].clone()
}

fn random_company(rng: &mut ThreadRng, size: u32) -> Company {
    let mut characters = Vec::new();
    for _ in 0..size {
        let character = random_player_character(rng, &characters);
        characters.push(character);
    }

    let mut relationships = Vec::new();
    for _ in 1..size {
        let relationship = loop {
            let relationship = random_relationship(rng);
            if !relationships.iter().any(|x| *x == relationship) {
                break relationship;
            }
        };
        relationships.push(relationship);
    }

    let relationships = relationships
        .into_iter()
        .enumerate()
        .map(|(i, x)| {
            format!(
                "{} and {} {}",
                characters[i].description.first_name,
                characters[i + 1].description.first_name,
                x
            )
        })
        .collect();

    let characters = characters
        .iter()
        .map(|x| {
            let mut profile = CharacterProfile::from(x);
            for asset in profile.assets.iter_mut() {
                if let Asset::Item(item) = asset {
                    if let ItemKind::PowerScroll(power_content) = &mut item.kind {
                        if let PowerContent::PowerOfKind(power_kind) = power_content {
                            match power_kind {
                                PowerKind::Sacred => {
                                    *power_content = PowerContent::Power(random_sacred_power(rng))
                                }
                                PowerKind::Sorcerous => {
                                    *power_content =
                                        PowerContent::Power(random_sorcerous_power(rng))
                                }
                            }
                        }
                    }
                }
            }
            profile
        })
        .collect();

    let additional_assets = vec![
        Asset::Item(Item::from(ItemKind::Candle)),
        Asset::Item(Item::from(ItemKind::FlintAndTinder)),
    ];

    Company {
        characters,
        relationships,
        additional_assets,
    }
}

fn random_relationship(rng: &mut ThreadRng) -> &'static str {
    random_element(rng, &relationships())
}

fn random_player_character(rng: &mut ThreadRng, others: &[PlayerCharacter]) -> PlayerCharacter {
    let abilities = random_abilities(rng);

    let background = loop {
        let background = random_background(rng);
        if !others.iter().any(|x| x.background == background) {
            break background;
        }
    };

    let starting_weapon = loop {
        let starting_weapon = random_starting_weapon(rng);
        if !others.iter().any(|x| x.starting_weapon == starting_weapon) {
            break starting_weapon;
        }
    };

    let starting_item = loop {
        let starting_item = random_starting_item(rng);
        if !others.iter().any(|x| x.starting_item == starting_item) {
            break starting_item;
        }
    };

    let starting_money = random_starting_money(rng);

    let gender = random_gender(rng);

    let age = random_age(rng);

    let reason_to_adventure = loop {
        let reason_to_adventure = random_reason_to_adventure(rng);
        if !others
            .iter()
            .any(|x| x.description.reason_to_adventure == reason_to_adventure)
        {
            break reason_to_adventure;
        }
    };

    let appearance = loop {
        let appearance = random_appearance(rng);
        if !others
            .iter()
            .any(|x| x.description.appearance == appearance)
        {
            break appearance;
        }
    };

    let personality = loop {
        let personality = random_personality(rng);
        if !others
            .iter()
            .any(|x| x.description.personality == personality)
        {
            break personality;
        }
    };

    let first_name = loop {
        let first_name = random_first_name(rng, gender);
        if !others
            .iter()
            .any(|x| x.description.first_name == first_name)
        {
            break first_name;
        }
    };

    let last_name = loop {
        let last_name = random_last_name(rng);
        if !others.iter().any(|x| x.description.last_name == last_name) {
            break last_name;
        }
    };

    PlayerCharacter {
        abilities,
        background,
        starting_weapon,
        starting_item,
        starting_money,
        description: PlayerDescription {
            gender,
            age,
            reason_to_adventure,
            appearance,
            personality,
            first_name,
            last_name,
        },
    }
}

fn random_abilities(rng: &mut ThreadRng) -> Abilities {
    let mut array = random_element(rng, Abilities::starting_arrays());
    array.shuffle(rng);
    Abilities::from(array)
}

fn random_background(rng: &mut ThreadRng) -> Background {
    random_element(rng, Background::all())
}

fn random_sacred_power(rng: &mut ThreadRng) -> Power {
    random_element(rng, Power::sacred_powers())
}

fn random_sorcerous_power(rng: &mut ThreadRng) -> Power {
    random_element(rng, Power::advanced_sorcerous_powers())
}

fn random_starting_weapon(rng: &mut ThreadRng) -> Item {
    random_element(rng, Item::starting_weapons())
}

fn random_starting_item(rng: &mut ThreadRng) -> Item {
    random_element(rng, Item::starting_items())
}

fn random_starting_money(rng: &mut ThreadRng) -> i32 {
    rng.gen_range(1..=8)
}

fn random_age(rng: &mut ThreadRng) -> i32 {
    5 + rng.gen_range(1..=4) * 10 + rng.gen_range(1..=10)
}

fn random_gender(rng: &mut ThreadRng) -> Gender {
    random_element(rng, Gender::all())
}

fn random_first_name(rng: &mut ThreadRng, gender: Gender) -> &'static str {
    random_element(rng, first_names(gender))
}

fn random_last_name(rng: &mut ThreadRng) -> &'static str {
    random_element(rng, last_names())
}

fn random_reason_to_adventure(rng: &mut ThreadRng) -> &'static str {
    random_element(rng, reasons_to_adventure())
}

fn random_appearance(rng: &mut ThreadRng) -> &'static str {
    random_element(rng, appearances())
}

fn random_personality(rng: &mut ThreadRng) -> &'static str {
    random_element(rng, personalities())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..=8), default_value_t = 1)]
    number: u32,

    #[arg(short, long, default_value = None, value_name = "FILE")]
    output_file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut rng = thread_rng();
    let company = random_company(&mut rng, args.number);

    match args.output_file {
        None => println!("{}", company.asciidoc()),
        Some(output_file) => {
            let mut file = File::create(output_file)?;
            writeln!(&mut file, "{}", company.asciidoc())?;
        }
    }

    Ok(())
}
