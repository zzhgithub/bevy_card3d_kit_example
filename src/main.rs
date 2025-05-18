pub mod card_info;
mod card_zone;
mod debug_lab;
mod game;
mod hand_card;
mod lua;
mod ui;
mod zone_info;

use crate::game::GamePlugin;
use crate::hand_card::CardLineResource;
use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::egui::TextStyle;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, Card3DPlugins, GamePlugin))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup);

    // 加载默认字体
    load_internal_binary_asset!(
        app,
        Handle::default(),
        "../assets/fonts/wqy-microhei.ttc",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.run();
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
}

//点击加入手牌
#[deprecated]
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
