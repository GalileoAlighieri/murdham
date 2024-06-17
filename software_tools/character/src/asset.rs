use super::{follower::*, item::*, item_kind::*, utils::*};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Asset {
    Item(Item),
    Follower(Follower),
    Money(i32),
}

impl From<Item> for Asset {
    fn from(value: Item) -> Self {
        Self::Item(value)
    }
}

impl From<Follower> for Asset {
    fn from(value: Follower) -> Self {
        Self::Follower(value)
    }
}

static_gen_fn!(starting_weapons, gen_starting_weapons, Item, Item, {
    vec![
        Item::new(Some("cleaver"), ItemKind::SimpleHandWeapon, None),
        Item::new(Some("club"), ItemKind::SimpleHandWeapon, None),
        Item::new(Some("dagger"), ItemKind::SimpleHandWeapon, None),
        Item::new(Some("hammer"), ItemKind::SimpleHandWeapon, None),
        Item::new(Some("hatchet"), ItemKind::SimpleHandWeapon, None),
        Item::new(Some("knife"), ItemKind::SimpleHandWeapon, None),
        Item::new(Some("sickle"), ItemKind::SimpleHandWeapon, None),
        Item::new(Some("bullwhip"), ItemKind::SimpleHandWeapon, None),
        Item::new(Some("boat hook"), ItemKind::SimpleGreatWeapon, None),
        Item::new(Some("pickaxe"), ItemKind::SimpleGreatWeapon, None),
        Item::new(Some("pitchfork"), ItemKind::SimpleGreatWeapon, None),
        Item::new(Some("sledgehammer"), ItemKind::SimpleGreatWeapon, None),
        Item::new(Some("staff"), ItemKind::SimpleGreatWeapon, None),
        Item::new(Some("woodcutting axe"), ItemKind::SimpleGreatWeapon, None),
        Item::new(Some("morningstar"), ItemKind::MartialHandWeapon, None),
        Item::new(Some("spear"), ItemKind::MartialHandWeapon, None),
        Item::new(Some("warhammer"), ItemKind::MartialHandWeapon, None),
        Item::new(
            Some("rusty zweihänder"),
            ItemKind::MartialGreatWeapon,
            Some("fragile"),
        ),
        Item::from(ItemKind::Mancatcher),
        Item::new(Some("blowgun"), ItemKind::MissileHandWeapon, None),
        Item::new(Some("darts"), ItemKind::MissileHandWeapon, None),
        Item::new(Some("sling"), ItemKind::MissileHandWeapon, None),
        Item::new(
            Some("makeshift bow"),
            ItemKind::MissileGreatWeapon,
            Some("fragile"),
        ),
        Item::new(Some("old pistol"), ItemKind::Handgun, Some("fragile")),
    ]
});

static_gen_fn!(starting_items, gen_starting_items, Item, Item, {
    vec![
        Item::from(ItemKind::AcidVial),
        Item::from(ItemKind::FlashPowder),
        Item::from(ItemKind::BearTrap),
        Item::from(ItemKind::Caltrops),
        Item::from(ItemKind::Helmet),
        Item::from(ItemKind::WarmClothes),
        Item::from(ItemKind::BeastmanBlood),
        Item::from(ItemKind::BlackAdderVenom),
        Item::from(ItemKind::CorpseMandrake),
        Item::from(ItemKind::MadcapMushroom),
        Item::new(None, ItemKind::Lantern, Some("empty")),
        Item::from(ItemKind::OilLamp),
        Item::from(ItemKind::ChessSet),
        Item::from(ItemKind::ClimbingGear),
        Item::from(ItemKind::FieldKitchen),
        Item::new(None, ItemKind::MusicInstrument, Some("flute")),
        Item::from(ItemKind::GrapplingHook),
        Item::from(ItemKind::LockPicks),
        Item::new(None, ItemKind::LuckyCharm, Some("cornicello")),
        Item::from(ItemKind::ManaclesAndKey),
        Item::from(ItemKind::MedicineBox),
        Item::from(ItemKind::Mirror),
        Item::from(ItemKind::Net),
        Item::from(ItemKind::PipeAndTobacco),
    ]
});
