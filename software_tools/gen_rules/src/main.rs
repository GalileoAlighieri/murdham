use asciidoc::*;
use character::*;
use clap::Parser;
use convert_case::{Case, Casing};
use profiles::*;
use std::{fs::File, io::Write, path::PathBuf};

const HEADER: &'static str = "// This file was automatically generated.\n";

fn domesticated_animals() -> Vec<CharacterProfile> {
    vec![
        camel(),
        cat(),
        cattle(),
        chicken(),
        small_dog(),
        medium_dog(),
        large_dog(),
        donkey(),
        duck(),
        goat(),
        light_horse(),
        heavy_horse(),
        mule(),
        pig(),
        pigeon(),
        pony(),
        rabbit(),
        sheep(),
    ]
}

fn wild_animals() -> Vec<CharacterProfile> {
    vec![
        alligator(),
        bat(),
        bear(),
        boar(),
        deer(),
        eagle(),
        elephant(),
        hawk(),
        lion(),
        lynx(),
        mouse(),
        owl(),
        rat(),
        raven(),
        tiger(),
        viper(),
        wolf(),
    ]
}

fn reanimated_corpses() -> Vec<CharacterProfile> {
    vec![bone_raven(), shambling_corpse(), skeletal_warrior()]
}

fn demons() -> Vec<CharacterProfile> {
    vec![bloodletter(), corruptor(), horror(), plague_brewer()]
}

fn monsters() -> Vec<CharacterProfile> {
    vec![troll(), wyrm()]
}

fn folks() -> Vec<CharacterProfile> {
    vec![human(), elf(), dwarf(), halfling(), ogre()]
}

fn write_list_section<T: AsciiDoc>(
    f: &mut File,
    title: Option<&str>,
    v: &[T],
    level: i8,
) -> std::io::Result<()> {
    if let Some(title) = title {
        writeln!(f, "{} {title}\n", vec!["="; level as usize].join(""))?;
    }
    for x in v.iter() {
        writeln!(f, "{}", x.asciidoc())?;
    }
    writeln!(f, "")?;
    Ok(())
}

fn gen_reference<T: AsciiDoc>(dir: &PathBuf, title: &str, v: &[T]) -> std::io::Result<()> {
    let tag = format!("ref_{}", title.to_case(Case::Snake));
    let mut file = File::create(dir.join(format!("{tag}.adoc")))?;

    writeln!(&mut file, "{HEADER}")?;

    write_list_section(&mut file, Some(title), v, 2)?;

    Ok(())
}

fn gen_reference_without_title<T: AsciiDoc>(
    dir: &PathBuf,
    title: &str,
    v: &[T],
) -> std::io::Result<()> {
    let tag = format!("ref_{}", title.to_case(Case::Snake));
    let mut file = File::create(dir.join(format!("{tag}.adoc")))?;

    writeln!(&mut file, "{HEADER}")?;

    write_list_section(&mut file, None, v, 3)?;

    Ok(())
}

trait TableFormatter {
    fn table_fmt(&self) -> String;
}

impl TableFormatter for &str {
    fn table_fmt(&self) -> String {
        String::from(*self)
    }
}

impl TableFormatter for Item {
    fn table_fmt(&self) -> String {
        let mut s = process_keywords(self.to_string());
        let money = ((16 - self.kind.cost()) / 2).max(0);
        if money > 0 {
            s += &format!(", {money}ʂ");
        }
        s
    }
}

impl TableFormatter for Background {
    fn table_fmt(&self) -> String {
        self.to_string()
    }
}

impl TableFormatter for Power {
    fn table_fmt(&self) -> String {
        self.to_string()
    }
}

impl TableFormatter for [i8; 3] {
    fn table_fmt(&self) -> String {
        format!("{}, {}, {}", self[0], self[1], self[2])
    }
}

fn gen_single_die_table<T: TableFormatter>(
    dir: &PathBuf,
    title: &str,
    col_header: &str,
    items: &[T],
    columns: usize,
) -> std::io::Result<()> {
    let tag = format!("tb_{}", title.to_case(Case::Snake));
    let mut file = File::create(dir.join(format!("{tag}.adoc")))?;

    writeln!(&mut file, "{HEADER}")?;

    let column_size = 12 / columns;

    writeln!(&mut file, ".{title}")?;
    writeln!(&mut file, "[[{tag}]]")?;
    writeln!(
        &mut file,
        r#"[options='header, unbreakable', cols="{}"]"#,
        vec![format!("^1,^{column_size}"); columns].join(",")
    )?;

    writeln!(&mut file, "|===")?;
    for _ in 0..columns {
        writeln!(&mut file, "|D{} |{col_header}", items.len())?;
    }

    for (i, x) in items.iter().enumerate() {
        writeln!(
            &mut file,
            "|{} |{}",
            i + 1,
            utils::capitalise(x.table_fmt())
        )?;
    }

    writeln!(&mut file, "|===")?;
    Ok(())
}

fn gen_double_die_table<T: TableFormatter>(
    dir: &PathBuf,
    title: &str,
    items: &[T],
    columns: usize,
) -> std::io::Result<()> {
    assert!(columns > 1);
    let tag = format!("tb_{}", title.to_case(Case::Snake));
    let mut file = File::create(dir.join(format!("{tag}.adoc")))?;

    writeln!(&mut file, "{HEADER}")?;

    let die_size = items.len() / columns;
    let column_size = 14 / columns;

    writeln!(&mut file, ".{title}")?;
    writeln!(&mut file, "[[{tag}]]")?;
    writeln!(
        &mut file,
        r#"[options='header, unbreakable', cols="^1h,{}"]"#,
        vec![format!("^{column_size}"); columns].join(",")
    )?;
    writeln!(&mut file, "|===")?;

    writeln!(&mut file, "h|  {columns}+h|D{columns}")?;
    writeln!(&mut file, "h|D{die_size}")?;
    for i in 1..=columns {
        write!(&mut file, " h|{i}")?;
    }
    write!(&mut file, "\n")?;

    for (i, x) in items.iter().enumerate() {
        if i % columns == 0 {
            writeln!(&mut file, "|{}", i / columns + 1)?;
        }
        writeln!(&mut file, "|{}", utils::capitalise(x.table_fmt()))?;
    }
    writeln!(&mut file, "|===")?;
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

    gen_reference(&args.output_dir, "Backgrounds", Background::all())?;
    gen_reference(&args.output_dir, "Skills", Skill::all())?;
    gen_reference(
        &args.output_dir,
        "Character categories",
        CharacterCategory::all(),
    )?;
    gen_reference(&args.output_dir, "Traits", Trait::all())?;
    gen_reference(&args.output_dir, "Conditions", Condition::all())?;

    gen_reference(&args.output_dir, "Item keywords", ItemKeyword::all())?;
    gen_reference(&args.output_dir, "Weapon keywords", WeaponKeyword::all())?;
    gen_reference(&args.output_dir, "Item modifiers", ItemModifier::all())?;
    gen_reference(&args.output_dir, "Weapons", ItemKind::weapons())?;
    gen_reference(&args.output_dir, "Artillery", ItemKind::artillery())?;
    gen_reference(&args.output_dir, "Vehicles", ItemKind::vehicles())?;
    gen_reference(&args.output_dir, "Armour", ItemKind::armour())?;
    gen_reference(&args.output_dir, "Gear", ItemKind::gear())?;
    gen_reference(&args.output_dir, "Item modifiers", ItemModifier::all())?;

    gen_reference(&args.output_dir, "Sacred powers", Power::sacred_powers())?;
    gen_reference(
        &args.output_dir,
        "Sorcerous powers",
        Power::sorcerous_powers(),
    )?;

    gen_reference_without_title(
        &args.output_dir,
        "Domesticated animals",
        &domesticated_animals(),
    )?;
    gen_reference_without_title(&args.output_dir, "Wild animals", &wild_animals())?;
    gen_reference_without_title(
        &args.output_dir,
        "Reanimated corpses",
        &reanimated_corpses(),
    )?;
    gen_reference_without_title(&args.output_dir, "Demons", &demons())?;
    gen_reference_without_title(&args.output_dir, "Monsters", &monsters())?;
    gen_reference_without_title(&args.output_dir, "Folks", &folks())?;

    gen_single_die_table(
        &args.output_dir,
        "Starting ability scores",
        "Scores",
        Abilities::starting_arrays(),
        3,
    )?;
    gen_single_die_table(
        &args.output_dir,
        "Sorcerous powers",
        "Power",
        Power::advanced_sorcerous_powers(),
        3,
    )?;
    gen_single_die_table(
        &args.output_dir,
        "Sacred powers",
        "Power",
        Power::sacred_powers(),
        3,
    )?;

    gen_double_die_table(&args.output_dir, "Backgrounds", Background::all(), 4)?;
    gen_double_die_table(
        &args.output_dir,
        "Starting weapons",
        Item::starting_weapons(),
        2,
    )?;
    gen_double_die_table(
        &args.output_dir,
        "Starting items",
        Item::starting_items(),
        2,
    )?;
    gen_double_die_table(&args.output_dir, "Appearance", character::appearances(), 2)?;
    gen_double_die_table(
        &args.output_dir,
        "Personality",
        character::personalities(),
        2,
    )?;
    gen_double_die_table(
        &args.output_dir,
        "Reason to adventure",
        character::reasons_to_adventure(),
        2,
    )?;
    gen_double_die_table(
        &args.output_dir,
        "Masculine names",
        character::masculine_first_names(),
        4,
    )?;
    gen_double_die_table(
        &args.output_dir,
        "Feminine names",
        character::feminine_first_names(),
        4,
    )?;
    gen_double_die_table(&args.output_dir, "Last names", character::last_names(), 4)?;
    gen_double_die_table(
        &args.output_dir,
        "Relationships",
        character::relationships(),
        2,
    )?;

    Ok(())
}
