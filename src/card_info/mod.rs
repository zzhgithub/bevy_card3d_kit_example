pub mod card_enums;

use crate::card_info::card_enums::{Attr, CardType, Race};
/// 卡片信息定义
use bevy::prelude::*;
use bevy_card3d_kit::prelude::*;

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct CardInfo {
    pub id: String,
    pub name: String,
    pub card_type: CardType,
    pub attr: Attr,
    pub race: Race,
    pub cost: usize,
    pub ack: u32,
}

impl CardMaterialGetter for CardInfo {
    fn get_face_mal(&self) -> String {
        format!("cards/{}.png", self.id)
    }

    fn get_back_mal(&self) -> String {
        format!("cards/{}.png", "back")
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

pub struct CardInfoPlugin;

impl Plugin for CardInfoPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CardInfo>();
        bind_card_render::<CardInfo>(app);
    }
}
