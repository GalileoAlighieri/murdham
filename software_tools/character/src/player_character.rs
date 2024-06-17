use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerDescription {
    pub gender: Gender,
    pub age: i32,
    pub reason_to_adventure: &'static str,
    pub appearance: &'static str,
    pub personality: &'static str,
    pub first_name: &'static str,
    pub last_name: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerCharacter {
    pub abilities: Abilities,
    pub background: Background,
    pub starting_weapon: Item,
    pub starting_item: Item,
    pub starting_money: i32,
    pub description: PlayerDescription,
}
