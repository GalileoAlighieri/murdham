use character::{utils::*, *};
use regex::Regex;
use std::collections::BTreeMap;

pub fn process_keywords(mut s: String) -> String {
    let re = Regex::new(r"~(?<s>[^~]*)~").unwrap();
    re.replace_all(&mut s, r"_${s}_").to_string()
}

fn collect_as_map<T: Clone + Ord>(v: &[T]) -> BTreeMap<T, u32> {
    let mut map = BTreeMap::new();
    for x in v.iter() {
        if let Some(count) = map.get_mut(x) {
            *count += 1;
        } else {
            map.insert(x.clone(), 1u32);
        }
    }
    map
}

pub trait AsciiDoc {
    fn asciidoc(&self) -> String;
}

impl<T> AsciiDoc for Vec<T>
where
    T: AsciiDoc,
{
    fn asciidoc(&self) -> String {
        let items: Vec<String> = self.iter().map(|x| x.asciidoc()).collect();
        items.join(", ")
    }
}

impl<T> AsciiDoc for BTreeMap<T, u32>
where
    T: AsciiDoc,
{
    fn asciidoc(&self) -> String {
        let items: Vec<String> = self
            .iter()
            .map(|(x, count)| {
                if *count > 1 {
                    format!("{}× {}", count, x.asciidoc())
                } else {
                    x.asciidoc()
                }
            })
            .collect();
        items.join(", ")
    }
}

impl AsciiDoc for Condition {
    fn asciidoc(&self) -> String {
        format!(
            "* *{}*.\n{}\n",
            capitalise(self.to_string()),
            process_keywords(String::from(self.description())),
        )
    }
}

impl AsciiDoc for Skill {
    fn asciidoc(&self) -> String {
        format!(
            "* *{}*.\n{}\n",
            capitalise(self.to_string()),
            process_keywords(self.description()),
        )
    }
}

impl AsciiDoc for ItemKeyword {
    fn asciidoc(&self) -> String {
        let name = match self {
            Self::Armour(_) => String::from("armour X"),
            Self::Durability(_, _) => String::from("durability X/Y"),
            Self::Vehicle(_) => String::from("vehicle X"),
            Self::Weapon(_) => String::from("weapon"),
            _ => self.to_string(),
        };
        format!(
            "* *{}*.\n{}\n",
            capitalise(name),
            process_keywords(String::from(self.description()))
        )
    }
}

impl AsciiDoc for WeaponKeyword {
    fn asciidoc(&self) -> String {
        let name = match self {
            Self::IndirectRange(_) => String::from("indirect range X"),
            Self::Range(_) => String::from("range X"),
            Self::Poison(_) => String::from("poison"),
            Self::UsageLimit(_) => String::from("usage limit"),
            _ => self.to_string(),
        };
        format!(
            "* *{}*.\n{}\n",
            capitalise(name),
            process_keywords(self.description())
        )
    }
}

impl AsciiDoc for ItemKind {
    fn asciidoc(&self) -> String {
        let cost_and_bulk = if self.bulk() == 0 {
            format!("{}ʂ, bulk ½", self.cost())
        } else if self.bulk() == 1 {
            format!("{}ʂ", self.cost())
        } else {
            format!("{}ʂ, bulk {}", self.cost(), self.bulk())
        };

        let mut components = Vec::new();
        components.push(format!(
            "* *{}* ({}).",
            capitalise(self.to_string()),
            cost_and_bulk
        ));

        if !self.keywords().is_empty() {
            let mut keywords = self.keywords();
            keywords.sort();
            let keywords = keywords
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    if i == 0 {
                        format!("_{}_", capitalise(x.to_string()))
                    } else {
                        format!("_{}_", x.to_string())
                    }
                })
                .collect::<Vec<String>>()
                .join(", ");
            components.push(format!("{}.", keywords));
        }

        if !self.description().is_empty() {
            components.push(process_keywords(self.description()));
        }

        components.join("\n") + "\n"
    }
}

impl AsciiDoc for ItemModifier {
    fn asciidoc(&self) -> String {
        let mut components = Vec::new();
        components.push(format!(
            "* *{}* ({}).",
            capitalise(self.to_string()),
            self.cost_modifier(),
        ));

        if !self.description().is_empty() {
            components.push(process_keywords(self.description()));
        }

        components.join("\n") + "\n"
    }
}

impl AsciiDoc for Item {
    fn asciidoc(&self) -> String {
        process_keywords(self.to_string())
    }
}

impl AsciiDoc for Follower {
    fn asciidoc(&self) -> String {
        process_keywords(self.to_string())
    }
}

impl AsciiDoc for Asset {
    fn asciidoc(&self) -> String {
        match self {
            Self::Item(x) => x.asciidoc(),
            Self::Follower(x) => x.asciidoc(),
            Self::Money(x) => format!("{}ʂ", x),
        }
    }
}

impl AsciiDoc for Background {
    fn asciidoc(&self) -> String {
        let mut skills = Vec::new();
        let mut advancements = Vec::new();
        for advancement in self.advancements().iter() {
            match advancement {
                Advancement::Skill(x) => {
                    skills.push(format!("_{x}_"));
                }
                Advancement::Abilities(x) => {
                    if x.str > 0 {
                        advancements.push(format!("STR+{}", x.str));
                    }
                    if x.agi > 0 {
                        advancements.push(format!("AGI+{}", x.agi));
                    }
                    if x.wit > 0 {
                        advancements.push(format!("WIT+{}", x.wit));
                    }
                }
                Advancement::Mana => advancements.push(String::from("mana+1")),
            }
        }

        let skills = if !skills.is_empty() {
            format!("*Skills*: {}.\n", skills.join(", "))
        } else {
            String::new()
        };

        let advancements = if !advancements.is_empty() {
            format!("*Advancements*: {}.\n", advancements.join(", "))
        } else {
            String::new()
        };

        format!(
            "* *{}*.\n{}\n{}{}*Assets*: {}.\n",
            capitalise(self.to_string()),
            process_keywords(String::from(self.description())),
            skills,
            advancements,
            process_keywords(collect_as_map(&self.assets()).asciidoc())
        )
    }
}

impl AsciiDoc for CharacterCategory {
    fn asciidoc(&self) -> String {
        format!(
            "* *{}*.\n{}\n",
            capitalise(self.to_string()),
            process_keywords(self.description())
        )
    }
}

impl AsciiDoc for Trait {
    fn asciidoc(&self) -> String {
        format!(
            "* *{}*.\n{}\n",
            capitalise(self.to_string()),
            process_keywords(self.description())
        )
    }
}

impl AsciiDoc for Power {
    fn asciidoc(&self) -> String {
        let mut components = Vec::new();

        components.push(format!("* *{}*.", capitalise(self.to_string())));

        components.push(if self.duration() == PowerDuration::Instant {
            format!("_{}_.", capitalise(self.range().to_string()))
        } else {
            format!(
                "_{}_, _{}_.",
                capitalise(self.range().to_string()),
                self.duration()
            )
        });

        components.push(format!("{}\n", process_keywords(self.description())));

        if !self.enhancements().is_empty() {
            components.push(
                self.enhancements()
                    .iter()
                    .map(|x| process_keywords(format!("** Level+{} -- {}\n", x.0, x.1)))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }

        components.join("\n")
    }
}

pub fn item_list(title: &str, items: &[ItemKind]) -> String {
    let items_str = items
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let mut name = x.to_string();
            if i == 0 {
                name = capitalise(name);
            }

            let mut components = Vec::new();

            if x.bulk() == 0 {
                components.push(String::from("bulk ½"))
            } else if x.bulk() > 1 {
                components.push(format!("bulk {}", x.bulk()))
            }

            for kw in x.keywords() {
                components.push(format!("_{}_", kw));
            }

            if !x.description().is_empty() {
                components.push(x.description())
            }

            if components.is_empty() {
                name
            } else {
                format!("{} ({})", name, components.join(", "))
            }
        })
        .collect::<Vec<String>>()
        .join(", ");
    format!("* *{title}* ({}ʂ).\n{}.\n", items[0].cost(), items_str)
}

impl AsciiDoc for Abilities {
    fn asciidoc(&self) -> String {
        format!("*STR* {}, *AGI* {}, *WIT* {}", self.str, self.agi, self.wit)
    }
}

impl AsciiDoc for Company {
    fn asciidoc(&self) -> String {
        let characters: Vec<String> = self.characters.iter().map(|x| x.asciidoc()).collect();
        let relationships: Vec<String> = self
            .relationships
            .iter()
            .map(|x| format!("* {}", x))
            .collect();
        let additional_assets: Vec<String> = self
            .additional_assets
            .iter()
            .map(|x| format!("* {}", capitalise(x.asciidoc())))
            .collect();
        let mut sections = vec![
            String::from("= The Company\n"),
            format!("== Characters\n\n{}\n", characters.join("\n")),
        ];
        if !relationships.is_empty() {
            sections.push(format!(
                "== Relationships\n\n{}\n",
                relationships.join("\n\n")
            ));
        }
        if !additional_assets.is_empty() {
            sections.push(format!(
                "== Additional assets\n\nThe company additionally starts with the following \
                 assets:\n\n{}\n",
                additional_assets.join("\n\n")
            ))
        }
        sections.join("\n")
    }
}

impl AsciiDoc for CharacterProfile {
    fn asciidoc(&self) -> String {
        let mut sections = Vec::new();

        let cost_string = if let Some(cost) = self.cost {
            format!(" ({cost}ʂ)")
        } else {
            String::new()
        };

        sections.push(format!(
            "*{}*{}",
            capitalise(self.name.clone()),
            cost_string
        ));

        if let Some(description) = &self.description {
            sections.push(format!("{}", capitalise(description.clone())));
        }

        let mana_string = if self.mana > 0 {
            format!(" -- *Mana* {}", self.mana)
        } else {
            String::new()
        };

        sections.push(format!("{}{}", self.abilities.asciidoc(), mana_string));

        sections.push(format!(
            "_{} {}_, _{}_.",
            capitalise(self.size.to_string()),
            self.category,
            self.intelligence,
        ));

        for skill in self.skills.iter() {
            sections.push(format!(
                "*{}*. {}",
                capitalise(skill.to_string()),
                process_keywords(skill.description())
            ));
        }

        for x in self.traits.iter() {
            sections.push(format!(
                "*{}*. {}",
                capitalise(x.to_string()),
                process_keywords(x.description())
            ));
        }

        if !self.assets.is_empty() {
            sections.push(process_keywords(format!(
                "*Assets*: {}.",
                collect_as_map(&self.assets).asciidoc(),
            )));
        }

        if let Some((name, av)) = self.natural_armour {
            sections.push(format!(
                "*{}*. Armour value {}.",
                capitalise(String::from(name)),
                av
            ));
        }

        for (name, weapon) in self.natural_weapons.iter() {
            sections.push(process_keywords(format!(
                "*{}*. {}.",
                capitalise(String::from(*name)),
                capitalise(weapon.to_string()),
            )));
        }

        for (rule, description) in self.special_rules.iter() {
            sections.push(format!(
                "*{}*. {}",
                capitalise(rule.clone()),
                process_keywords(capitalise(description.clone()))
            ));
        }

        sections.push(String::from("\n"));

        sections.join(" +\n")
    }
}
