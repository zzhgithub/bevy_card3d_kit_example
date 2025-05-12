use crate::card_info::CardInfoPlugin;
use crate::card_zone::can_set::CardSetZonePlugin;
use crate::debug_lab::DebugLabPlugin;
use crate::hand_card::HandCardPlugin;
use crate::lua::LuaPlugin;
use crate::ui::ShowDialogPlugin;
use crate::zone_info::ZoneInfoPlugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CardInfoPlugin,
            ZoneInfoPlugin,
            HandCardPlugin,
            LuaPlugin,
            CardSetZonePlugin,
            DebugLabPlugin,
            ShowDialogPlugin,
        ));
    }
}
