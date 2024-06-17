use super::{utils::capitalise, *};
use convert_case::{Case, Casing};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharacterProfile {
    pub name: String,
    pub cost: Option<i32>,
    pub abilities: Abilities,
    pub mana: i8,
    pub category: CharacterCategory,
    pub size: SizeCategory,
    pub intelligence: IntelligenceCategory,
    pub skills: Vec<Skill>,
    pub traits: Vec<Trait>,
    pub assets: Vec<Asset>,
    pub natural_armour: Option<(&'static str, i8)>,
    pub natural_weapons: Vec<(&'static str, Weapon)>,
    pub description: Option<String>,
    pub special_rules: Vec<(String, String)>,
}

impl From<&PlayerCharacter> for CharacterProfile {
    fn from(pc: &PlayerCharacter) -> Self {
        let name = format!(
            "{} {} the {}",
            pc.description.first_name,
            pc.description.last_name,
            pc.background
                .gender_specific_name(pc.description.gender)
                .to_case(Case::Title)
        );

        let cost = None;

        let abilities = {
            let mut abilities = pc.abilities.clone();
            for advancement in pc.background.advancements().iter() {
                if let Advancement::Abilities(x) = advancement {
                    abilities += x;
                }
            }
            abilities
        };

        let mana = pc
            .background
            .advancements()
            .iter()
            .filter(|x| match x {
                Advancement::Mana => true,
                _ => false,
            })
            .count() as i8;

        let category = CharacterCategory::Creature;
        let size = SizeCategory::MediumSized;
        let intelligence = IntelligenceCategory::HumanIntelligence;

        let skills = pc
            .background
            .advancements()
            .iter()
            .filter(|x| match x {
                Advancement::Skill(_) => true,
                _ => false,
            })
            .map(|x| match x {
                Advancement::Skill(x) => *x,
                _ => panic!("Unreachable"),
            })
            .collect::<Vec<Skill>>();

        let assets = {
            let mut assets = pc.background.assets();
            assets.push(Asset::Item(pc.starting_weapon.clone()));
            assets.push(Asset::Item(pc.starting_item.clone()));

            let money = pc.starting_money
                + ((16 - pc.starting_weapon.kind.cost()) / 2).max(0)
                + ((16 - pc.starting_item.kind.cost()) / 2).max(0);
            assets.push(Asset::Money(money));
            assets.sort();
            assets
        };

        let description = Some(format!(
            "{}, {} years old. {} You have abandoned your previous life because {} {} {}",
            capitalise(pc.description.gender.to_string()),
            pc.description.age,
            pc.background.description(),
            pc.description.reason_to_adventure,
            pc.description.appearance,
            pc.description.personality
        ));

        let traits = Vec::new();
        let natural_armour = None;
        let natural_weapons = Vec::new();
        let special_rules = Vec::new();
        Self {
            name,
            cost,
            abilities,
            mana,
            category,
            size,
            intelligence,
            skills,
            traits,
            assets,
            natural_armour,
            natural_weapons,
            description,
            special_rules,
        }
    }
}
