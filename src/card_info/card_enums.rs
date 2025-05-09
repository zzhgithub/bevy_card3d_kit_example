use bevy::prelude::Reflect;
use std::str::FromStr;

#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum CardType {
    // 人物
    Actor,
    // 神秘术
    Arcane,
    // 模因
    Meme,
}

impl FromStr for CardType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Actor" => Ok(CardType::Actor),
            "Arcane" => Ok(CardType::Arcane),
            "Meme" => Ok(CardType::Meme),
            _ => Err(()),
        }
    }
}
//属性
#[derive(Debug, Clone, Reflect)]
pub enum Attr {
    /**
     * 星
     */
    STAR,
    /**
     * 兽
     */
    BEAST,
    /**
     * 木
     */
    PLANT,
    /**
     * 岩
     */
    MINERAL,
    /**
     * 灵
     */
    SPIRIT,
    /**
     * 智
     */
    INTELLECT,
}

impl FromStr for Attr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "STAR" => Ok(Attr::STAR),
            "BEAST" => Ok(Attr::BEAST),
            "PLANT" => Ok(Attr::PLANT),
            "MINERAL" => Ok(Attr::MINERAL),
            "SPIRIT" => Ok(Attr::SPIRIT),
            "INTELLECT" => Ok(Attr::INTELLECT),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub enum Race {
    // 没有种族
    NULL,
    /**
     * 神秘学家
     */
    Arcanist,
    /**
     * 超自然者
     */
    Beyond,
    /**
     * 意识唤醒者
     */
    Awakened,
    /**
     * 混血种
     */
    Mixed,
    /**
     * 人类
     */
    Human,
}

impl FromStr for Race {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NULL" => Ok(Race::NULL),
            "Arcanist" => Ok(Race::Arcanist),
            "Beyond" => Ok(Race::Beyond),
            "Awakened" => Ok(Race::Awakened),
            "Mixed" => Ok(Race::Mixed),
            "Human" => Ok(Race::Human),
            _ => Err(()),
        }
    }
}
