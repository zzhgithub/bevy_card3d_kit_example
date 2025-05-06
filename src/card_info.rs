/// 卡片信息定义
use bevy::prelude::*;
use bevy_card3d_kit::prelude::*;

#[derive(Component, Clone)]
pub struct CardInfo {
    pub id: String,
    pub name: String,
}

impl CardMaterialGetter for CardInfo {
    fn get_face_mal(&self) -> String {
        format!("cards/{}.png", self.id)
    }

    fn get_back_mal(&self) -> String {
        format!("cards/{}.png", "back")
    }
}

pub struct CardInfoPlugin;

impl Plugin for CardInfoPlugin {
    fn build(&self, app: &mut App) {
        bind_card_render::<CardInfo>(app);
    }
}
