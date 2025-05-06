use crate::card_info::{CardInfo, CardInfoPlugins};
use crate::hand_card::HandCardPlugin;
use crate::zone_info::ZoneInfoPlugin;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::card_state::CardState;
use bevy_card3d_kit::prelude::{CardLine, HAND_CARD_LEVEL};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CardInfoPlugins, ZoneInfoPlugin, HandCardPlugin));
    }
}
