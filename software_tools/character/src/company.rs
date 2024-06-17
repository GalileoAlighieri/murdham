use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Company {
    pub characters: Vec<CharacterProfile>,
    pub relationships: Vec<String>,
    pub additional_assets: Vec<Asset>,
}
