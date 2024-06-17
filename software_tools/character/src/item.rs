use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    pub kind: ItemKind,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Item {
    pub fn new<T: Into<String>>(name: Option<T>, kind: ItemKind, description: Option<T>) -> Self {
        Self {
            name: name.map(|x| x.into()),
            kind,
            description: description.map(|x| x.into()),
        }
    }

    pub fn keywords(&self) -> Option<String> {
        let mut components = self.kind.keywords().clone();
        components.sort();

        let components = components
            .iter()
            .map(|x| match x {
                ItemKeyword::Weapon(weapon) => weapon.to_string(),
                _ => format!("{x}"),
            })
            .collect::<Vec<String>>()
            .join(", ");

        if !components.is_empty() {
            Some(components)
        } else {
            None
        }
    }
}

impl From<ItemKind> for Item {
    fn from(kind: ItemKind) -> Self {
        Self {
            kind,
            name: None,
            description: None,
        }
    }
}

pub fn format_bulk(bulk: i32) -> String {
    if bulk == 0 {
        String::from("bulk ½")
    } else {
        format!("bulk {bulk}")
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.clone().unwrap_or(self.kind.to_string());

        write!(f, "{name}")?;

        let mut components = Vec::new();
        if self.kind.bulk() != 1 {
            components.push(format_bulk(self.kind.bulk()));
        }
        if let Some(keywords) = self.keywords() {
            components.push(keywords);
        }
        if let Some(description) = &self.description {
            components.push(String::from(description));
        }
        if !components.is_empty() {
            write!(f, " ({})", components.join("; "))?;
        }

        Ok(())
    }
}
