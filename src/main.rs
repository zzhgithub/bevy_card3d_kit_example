mod card_info;
mod game;
mod zone_info;
mod hand_card;

use crate::card_info::{CardInfo, CardInfoPlugins};
use crate::game::GamePlugin;
use crate::zone_info::ZoneInfoPlugin;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins, GamePlugin))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // 相机
    commands.spawn((
        SharkCamera,
        Camera3d::default(),
        Msaa::Sample4,
        Transform::from_xyz(0., 0., 25.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 光源
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    // commands.spawn((
    //     CardInfo {
    //         id: "NAAI-A-001".to_string(),
    //         name: "NAAI-A-001".to_string(),
    //     },
    //     Card {
    //         origin: Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL),
    //     },
    // ));
}
