use character::{utils::capitalise, *};
use clap::Parser;
use regex::Regex;
use std::{fs::File, io::Write, path::PathBuf};

const HEADER: &'static str = "// This file was automatically generated.\n";

fn process_keywords(mut s: String) -> String {
    let re = Regex::new(r"~(?<s>[^~]*)~").unwrap();
    re.replace_all(&mut s, r"<i>${s}</i>")
        .to_string()
        .replace("'`", "'")
        .replace("`'", "'")
}

fn backgrounds(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const backgrounds = [")?;
    for background in character::Background::all().iter() {
        let mut abilities = Abilities::new(0, 0, 0);
        let mut mana = 0;
        let mut skills = Vec::new();
        for advancement in background.advancements() {
            match advancement {
                Advancement::Abilities(v) => abilities = v,
                Advancement::Mana => mana += 1,
                Advancement::Skill(v) => skills.push(v),
            }
        }

        let mut money = 0;
        let mut items = Vec::new();
        let mut followers = Vec::new();
        let mut sorcerous_scrolls = 0;
        let mut sacred_scrolls = 0;
        for asset in background.assets() {
            match asset {
                Asset::Item(v) => {
                    if let ItemKind::PowerScroll(power_content) = v.kind {
                        if let PowerContent::PowerOfKind(power_kind) = power_content {
                            match power_kind {
                                PowerKind::Sacred => {
                                    sacred_scrolls += 1;
                                }
                                PowerKind::Sorcerous => {
                                    sorcerous_scrolls += 1;
                                }
                            }
                        } else {
                            items.push(v)
                        }
                    } else {
                        items.push(v)
                    }
                }
                Asset::Follower(v) => followers.push(v),
                Asset::Money(v) => money += v,
            }
        }

        writeln!(f, "{{")?;
        writeln!(
            f,
            "masculine_name: \"{}\",",
            capitalise(background.gender_specific_name(Gender::Male))
        )?;
        writeln!(
            f,
            "feminine_name: \"{}\",",
            capitalise(background.gender_specific_name(Gender::Female))
        )?;
        writeln!(f, "description: \"{}\",", background.description())?;
        writeln!(
            f,
            "abilities: [{}, {}, {}],",
            abilities.str, abilities.agi, abilities.wit
        )?;
        writeln!(f, "skills: {{")?;
        for skill in skills.iter() {
            writeln!(
                f,
                "\"{}\": \"{}\",",
                capitalise(skill.to_string()),
                process_keywords(skill.description())
            )?;
        }
        writeln!(f, "}},")?;
        writeln!(f, "mana: {},", mana)?;
        writeln!(f, "assets: [")?;
        for item in items.iter() {
            writeln!(f, "\"{}\",", process_keywords(item.to_string()))?;
        }
        for follower in followers.iter() {
            writeln!(f, "\"{}\",", process_keywords(follower.to_string()))?;
        }
        writeln!(f, "],")?;
        writeln!(f, "sacred_scrolls: {},", sacred_scrolls)?;
        writeln!(f, "sorcerous_scrolls: {},", sorcerous_scrolls)?;
        writeln!(f, "money: {},", money)?;
        writeln!(f, "}},")?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

fn weapons(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const weapons = [")?;
    for v in Item::starting_weapons().iter() {
        writeln!(
            f,
            "[\"{}\", {}],",
            process_keywords(v.to_string()),
            0.max(16 - v.kind.cost()) / 2
        )?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

fn items(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const items = [")?;
    for v in Item::starting_items().iter() {
        writeln!(
            f,
            "[\"{}\", {}],",
            process_keywords(v.to_string()),
            0.max(16 - v.kind.cost()) / 2
        )?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

fn sacred_scrolls(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const sacred_scrolls = [")?;
    for v in Power::sacred_powers() {
        let scroll = Item::from(ItemKind::PowerScroll(PowerContent::Power(*v)));
        writeln!(f, "\"{}\",", process_keywords(scroll.to_string()))?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

fn sorcerous_scrolls(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const sorcerous_scrolls = [")?;
    for v in Power::advanced_sorcerous_powers() {
        let scroll = Item::from(ItemKind::PowerScroll(PowerContent::Power(*v)));
        writeln!(f, "\"{}\",", process_keywords(scroll.to_string()))?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

fn genders(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const genders = [")?;
    for v in ["Male", "Female"] {
        writeln!(f, "\"{v}\",")?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

fn goals(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const goals = [")?;
    for v in character::reasons_to_adventure().iter() {
        writeln!(f, "\"{v}\",")?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

fn appearances(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const appearances = [")?;
    for v in character::appearances().iter() {
        writeln!(f, "\"{v}\",")?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

fn personalities(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const personalities = [")?;
    for v in character::personalities().iter() {
        writeln!(f, "\"{v}\",")?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

fn masculine_first_names(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const masculine_first_names = [")?;
    for v in character::masculine_first_names().iter() {
        writeln!(f, "\"{v}\",")?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

fn feminine_first_names(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const feminine_first_names = [")?;
    for v in character::feminine_first_names().iter() {
        writeln!(f, "\"{v}\",")?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

fn last_names(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "export const last_names = [")?;
    for v in character::last_names().iter() {
        writeln!(f, "\"{v}\",")?;
    }
    writeln!(f, "];\n")?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    output_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut file = File::create(args.output_dir)?;
    writeln!(&mut file, "{HEADER}")?;
    backgrounds(&mut file)?;
    weapons(&mut file)?;
    items(&mut file)?;
    sacred_scrolls(&mut file)?;
    sorcerous_scrolls(&mut file)?;
    genders(&mut file)?;
    goals(&mut file)?;
    appearances(&mut file)?;
    personalities(&mut file)?;
    masculine_first_names(&mut file)?;
    feminine_first_names(&mut file)?;
    last_names(&mut file)?;

    Ok(())
}
