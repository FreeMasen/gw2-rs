use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use std::collections::HashMap;

pub mod account;
pub mod achievements;
pub mod backstory;
pub mod characters;
pub mod commerce;
pub mod emblems;
pub mod floors;
pub mod guild;
pub mod pvp;
pub mod skills;
pub mod specializations;
pub mod stories;
pub mod token_info;
pub mod traits;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cat {
    pub id: u64,
    pub hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub id: u64,
    pub name: String,
    pub base_rgb: [u8; 3],
    pub cloth: MaterialColor,
    pub leather: MaterialColor,
    pub metal: MaterialColor,
    pub fur: MaterialColor,
    pub item: Option<u64>,
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialColor {
    pub brightness: i32,
    pub contrast: f64,
    pub hue: u32,
    pub saturation: f64,
    pub lightness: f64,
    pub rgb: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub order: u64,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dungeon {
    pub id: String,
    pub paths: Vec<DungeonPath>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonPath {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finisher {
    pub id: u64,
    pub unlock_details: String,
    pub unlock_items: Vec<u64>,
    pub order: u64,
    pub icon: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glider {
    pub id: u64,
    pub unlock_items: Vec<u64>,
    pub order: u64,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub default_dyes: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    // #[serde(rename = "type")]
    // pub kind: _ItemDetail,
    pub level: u64,
    pub rarity: String,
    pub vender_value: Option<u64>,
    pub game_types: Vec<String>,
    pub flags: Vec<String>,
    pub restrictions: Vec<String>,
    pub chat_link: String,
    pub icon: String,
    #[serde(flatten)]
    pub details: _ItemDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "details")]
pub enum _ItemDetail {
    Armor {
        #[serde(rename = "type")]
        kind: ArmorKind,
        #[serde(flatten)]
        details: ArmorDetails,
    },
    Back {
        #[serde(default)]
        infusion_slots: Vec<Infusion>,
        attribute_adjustment: Option<f64>,
        infix_upgrade: Option<Upgrade>,
        suffix_item_id: Option<u64>,
        #[serde(default)]
        secondary_suffix_item_id: String,
        #[serde(default)]
        stat_choices: Vec<u64>,
    },
    Bag {
        size: u64,
        no_sell_or_sort: bool,
    },
    Consumable {
        #[serde(rename = "type")]
        kind: ConsumableKind,
        description: Option<String>,
        duration_ms: Option<u64>,
        unlock_type: Option<UnlockType>,
        color_id: Option<u64>,
        recipe_id: Option<u64>,
        #[serde(default)]
        extra_recipe_ids: Vec<u64>,
        guild_upgrade_id: Option<u64>,
        apply_count: Option<u64>,
        name: Option<String>,
        icon: Option<String>,
        #[serde(default)]
        skins: Vec<u64>,
    },
    Container {
        #[serde(rename = "type")]
        kind: ContainerKind,
    },
    CraftingMaterial,
    Gathering {
        #[serde(rename = "type")]
        kind: GatheringKind,
    },
    Gizmo {
        #[serde(rename = "type")]
        kind: GizmoKind,
        guild_upgrade_id: Option<u64>,
        #[serde(default)]
        vender_ids: Vec<u64>,
    },
    Key,
    MiniPet {
        minipet_id: u64,
    },
    Tool,
    Trait,
    Trinket {
        #[serde(rename = "type")]
        kind: TrinketKind,
        #[serde(default)]
        infusion_slots: Vec<Infusion>,
        attribute_adjustment: Option<f64>,
        infix_upgrade: Option<InfixUpgrade>,
        suffix_item_id: Option<u64>,
        #[serde(default)]
        secondary_suffix_item_id: String,
        #[serde(default)]
        stat_choices: Vec<u64>,
    },
    Trophy,
    UpgradeComponent {
        #[serde(rename = "type")]
        kind: UpgradeKind,
        #[serde(default)]
        flags: Vec<UpgradeFlag>,
        #[serde(default)]
        infusion_upgrade_flags: Vec<InfusionUpgradeFlag>,
        #[serde(default)]
        suffix: String,
        infix_upgrade: Option<InfixUpgrade>,
        #[serde(default)]
        bonuses: Vec<String>,
    },
    Weapon {
        #[serde(rename = "type")]
        kind: WeaponKind,
        #[serde(rename = "damage_type")]
        damage_kind: DamageKind,
        min_power: u64,
        max_power: u64,
        defense: u64,
        #[serde(default)]
        infusion_slots: Vec<Infusion>,
        attribute_adjustment: Option<f64>,
        infix_upgrade: Option<InfixUpgrade>,
        suffix_item_id: Option<u64>,
        #[serde(default)]
        secondary_suffix_item_id: String,
        #[serde(default)]
        stat_choices: Vec<u64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorKind {
    Helm,
    Shoulders,
    Coat,
    Gloves,
    Leggings,
    Boots,
    HelmAquatic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsumableKind {
    AppearanceChange,
    Booze,
    ContractNpc,
    Currency,
    Food,
    Generic,
    Halloween,
    Immediate,
    MountRandomUnlock,
    RandomUnlock,
    Transmutation,
    Unlock,
    UpgradeRemoval,
    Utility,
    TeleportToFriend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnlockType {
    BagSlot,
    BankTab,
    Champion,
    CollectibleCapacity,
    Content,
    CraftingRecipe,
    Dye,
    GliderSkin,
    #[serde(rename = "Minipet")]
    MiniPet,
    #[serde(rename = "Ms")]
    MountSkin,
    Outfit,
    RandomUnlock,
    SharedSlot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerKind {
    Default,
    GiftBox,
    Immediate,
    #[serde(rename = "OpenUI")]
    OpenUi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GatheringKind {
    Foraging,
    Logging,
    Mining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GizmoKind {
    Default,
    ContainerKey,
    RentableContractNpc,
    UnlimitedConsumable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrinketKind {
    Accessory,
    Amulet,
    Ring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpgradeKind {
    Default,
    Gem,
    Rune,
    Sigil,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpgradeFlag {
    Axe,
    Dagger,
    Focus,
    Greatsword,
    Hammer,
    Harpoon,
    LongBow,
    Mace,
    Pistol,
    Rifle,
    Scepter,
    Shield,
    ShortBow,
    Speargun,
    Staff,
    Sword,
    Torch,
    Trident,
    Warhorn,
    HeavyArmor,
    MediumArmor,
    LightArmor,
    Trinket,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfusionUpgradeFlag {
    Enrichment,
    Infusion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorDetails {
    pub weight_class: ArmorClass,
    pub defense: u64,
    #[serde(default)]
    pub infusion_slots: Vec<Infusion>,
    pub attribute_adjustment: Option<f64>,
    pub infix_upgrade: Option<InfixUpgrade>,
    pub suffix_item_id: Option<u64>,
    #[serde(default)]
    pub secondary_suffix_item_id: String,
    #[serde(default)]
    pub stat_choices: Vec<u64>,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone)]
pub enum ArmorClass {
    Heavy,
    Medium,
    Light,
    Clothing,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone)]
pub enum ItemKind {
    Armor,
    Back,
    Bag,
    Consumable,
    Container,
    CraftingMaterial,
    Gathering,
    Gizmo,
    Key,
    MiniPet,
    Tool,
    Trait,
    Trinket,
    Trophy,
    UpgradeComponent,
    Weapon,
    #[serde(other)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemDetails {
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub damage_type: Option<String>,
    pub min_power: Option<u64>,
    pub max_power: Option<u64>,
    pub defense: Option<u64>,
    #[serde(default)]
    pub infusion_slots: Vec<Infusion>,
    pub attribute_adjustment: Option<f64>,
    pub secondary_suffix_item_id: Option<String>,
    pub infix_upgrade: Option<Upgrade>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfixUpgrade {
    pub id: u64,
    pub attributes: Vec<InfixUpgradeAttribute>,
    pub buff: Option<InfixUpgradeAttributeBuff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfixUpgradeAttribute {
    pub attribute: InfixUpgradeAttributeAttribute,
    pub modifier: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfixUpgradeAttributeAttribute {
    AgonyResistance,
    BoonDuration,
    ConditionDamage,
    ConditionDuration,
    CritDamage,
    Healing,
    Power,
    Precision,
    Toughness,
    Vitality,
}

impl std::fmt::Display for InfixUpgradeAttributeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InfixUpgradeAttributeAttribute::AgonyResistance => "Agony Resistance",
                InfixUpgradeAttributeAttribute::BoonDuration => "Concentration",
                InfixUpgradeAttributeAttribute::ConditionDamage => "Condition Damage",
                InfixUpgradeAttributeAttribute::ConditionDuration => "Expertise",
                InfixUpgradeAttributeAttribute::CritDamage => "Ferocity",
                InfixUpgradeAttributeAttribute::Healing => "Healing",
                InfixUpgradeAttributeAttribute::Power => "Power",
                InfixUpgradeAttributeAttribute::Precision => "Precision",
                InfixUpgradeAttributeAttribute::Toughness => "Toughness",
                InfixUpgradeAttributeAttribute::Vitality => "Vitality",
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfixUpgradeAttributeBuff {
    pub skill_id: u64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponKind {
    Axe,
    Dagger,
    Mace,
    Pistol,
    Scepter,
    Sword,
    Focus,
    Shield,
    Torch,
    Warhorn,
    Greatsword,
    Happer,
    LongBow,
    Rifle,
    ShortBow,
    Staff,
    Harpoon,
    Speargun,
    Trident,
    LargeBundle,
    SmallBundle,
    Toy,
    TwoTwoHand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DamageKind {
    Fire,
    Ice,
    Lightning,
    Physical,
    Choking,
}

impl Eq for ItemDetails {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Infusion {
    flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Upgrade {
    pub id: u64,
    pub attributes: Vec<UpgradeAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpgradeAttribute {
    pub attribute: String,
    pub modifier: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStats {
    pub name: Option<String>,
    pub attributes: HashMap<String, f64>,
    pub id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Legend {
    pub id: u64,
    pub swap: u64,
    pub heal: u64,
    pub elite: u64,
    pub utilities: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailCarrier {
    pub id: u64,
    pub unlock_items: Vec<u64>,
    pub order: u64,
    pub icon: String,
    pub name: String,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mastery {
    pub id: u64,
    pub name: String,
    pub requirement: String,
    pub order: u64,
    pub background: String,
    pub region: String,
    pub levels: Vec<MasteryLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryLevel {
    pub name: String,
    pub description: String,
    pub instruction: String,
    pub icon: String,
    pub point_cost: u8,
    pub exp_cost: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub id: u64,
    pub name: String,
    pub order: u64,
    pub items: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mini {
    pub id: u64,
    pub name: String,
    pub icon: String,
    pub item_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outfit {
    pub id: u64,
    pub name: String,
    pub icon: String,
    pub unlock_items: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pet {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub skills: Vec<PetSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetSkill {
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profession {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub icon_big: String,
    pub specialization: Vec<u64>,
    pub training: Vec<ProfessionTraining>,
    pub weapons: HashMap<String, Weapon>,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfessionTraining {
    pub id: u64,
    pub category: String,
    pub name: String,
    pub track: TrainingTrack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingTrack {
    pub cost: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub trait_id: Option<u64>,
    pub skill_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub flags: Vec<String>,
    pub skills: Vec<Skill>,
    pub specialization: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: u64,
    pub slot: String,
    pub offhand: Option<String>,
    pub attunement: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Race {
    pub id: String,
    pub name: String,
    pub skills: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Raid {
    pub id: String,
    pub wings: Vec<RaidWing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidWing {
    pub id: String,
    pub events: Vec<RaidWingEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidWingEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: u64,
    pub chat_link: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub output_item_id: u64,
    pub output_item_count: u64,
    pub min_rating: u64,
    pub time_to_craft_ms: u64,
    pub disciplines: Vec<String>,
    pub flags: Vec<String>,
    pub ingredients: Vec<RecipeIngredient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub item_id: u64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skin {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub icon: String,
    pub flags: Vec<String>,
    pub rarity: String,
    pub restrictions: Vec<String>,
    pub details: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: u64,
    pub name: String,
    pub achievement: Option<u64>,
    pub achievements: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub id: u64,
    pub name: String,
    pub population: String,
}
