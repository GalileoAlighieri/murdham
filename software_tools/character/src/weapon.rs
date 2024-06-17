use super::{die::*, weapon_keyword::*};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Weapon {
    pub damage: Option<Die>,
    pub keywords: Vec<WeaponKeyword>,
}

impl Weapon {
    pub fn new(damage: Option<Die>, keywords: Vec<WeaponKeyword>) -> Self {
        Self { damage, keywords }
    }
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            damage: Some(Die::D4),
            keywords: Vec::new(),
        }
    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut entries = Vec::new();
        if let Some(damage) = self.damage {
            entries.push(format!("{} damage", damage.to_string()));
        }
        for kw in self.keywords.iter() {
            entries.push(format!("{kw}"));
        }
        write!(f, "{}", entries.join(", "))
    }
}
