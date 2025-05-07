mod card_info;
mod game;
mod hand_card;
mod lua;
mod zone_info;

use crate::game::GamePlugin;
use crate::hand_card::CardLineResource;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_scriptum::Script;
use bevy_scriptum::runtimes::lua::LuaScript;

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    commands
        .spawn((
            Script::<LuaScript>::new(asset_server.load("lua/NAAI-A-001.lua")),
            Card {
                origin: Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL),
            },
        ))
        .observe(on_click);

    commands
        .spawn((
            Script::<LuaScript>::new(asset_server.load("lua/EX001-A-002.lua")),
            Card {
                origin: Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL),
            },
        ))
        .observe(on_click);
}

//点击加入手牌
fn on_click(
    click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    card_line_resource: Res<CardLineResource>,
) {
    commands
        .entity(click.target())
        .insert(HandCard {
            belong_to_card_line: Some(card_line_resource.my_card_line),
        })
        .insert(Moveable);
}
