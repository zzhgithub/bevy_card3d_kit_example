use crate::hand_card::CardLineResource;
use crate::zone_info::AllZoneInfoResource;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::card_state::CardState;
use bevy_card3d_kit::prelude::{Card, HandCard, Moveable};
use bevy_card3d_kit::zone::desk_zone::{DeskCard, DeskZone, DeskZoneChangedEvent};
use bevy_scriptum::Script;
use bevy_scriptum::runtimes::lua::LuaScript;

pub struct DebugLabPlugin;

impl Plugin for DebugLabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.0)),
                        row_gap: Val::Px(5.0),
                        column_gap: Val::Px(5.0),
                        ..Default::default()
                    },
                    BackgroundColor(bevy::color::palettes::css::DARK_GRAY.into()),
                ))
                .with_children(|parent| {
                    spawn_button(parent, "init desk".to_string(), on_click_init_desk);
                    spawn_button(parent, "draw".to_string(), on_click_draw);
                });
        });
}

fn spawn_button<E: Event, B: Bundle, M>(
    mut parent: &mut RelatedSpawnerCommands<ChildOf>,
    text: String,
    observer: impl IntoObserverSystem<E, B, M>,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(60.),
                height: Val::Px(21.),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center, // 按钮文字居中
                align_items: AlignItems::Center,         // 垂直居中
                border: UiRect::all(Val::Px(2.0)),       // 白色边框
                ..Default::default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(bevy::color::palettes::css::LIGHT_GRAY.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    // font: font.clone(),
                    font_size: 10.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        })
        .observe(observer);
}

fn on_click_init_desk(
    _click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    all_zone_info_resource: Res<AllZoneInfoResource>,
    asset_server: Res<AssetServer>,
) {
    info!("Clicked on pointer");
    // 这里是测试的卡片的代码
    for card_num in "S001-A-001,EX001-A-002,NAAI-A-001,S001-T-001,S001-M-001".split(",") {
        commands.spawn((
            Script::<LuaScript>::new(asset_server.load(format!("lua/{}.lua", card_num))),
            Card {
                origin: Transform::default(),
            },
            CardState {
                face_up: false,
                vertical: true,
            },
            DeskCard {
                belongs_to_desk: Some(all_zone_info_resource.my.desk.clone()),
            },
        ));
    }
}

fn on_click_draw(
    _click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    all_zone_info_resource: Res<AllZoneInfoResource>,
    card_line_resource: Res<CardLineResource>,
    mut query_desks: Query<&mut DeskZone>,
    mut desk_card_event: EventWriter<DeskZoneChangedEvent>,
) {
    if let Ok(mut desk_zone) = query_desks.get_mut(all_zone_info_resource.my.desk) {
        if let Some(card_entity) = desk_zone.card_list.pop() {
            commands
                .entity(card_entity)
                .insert(HandCard {
                    belong_to_card_line: Some(card_line_resource.my_card_line),
                })
                .insert(Moveable);
            desk_card_event.write(DeskZoneChangedEvent::Removed {
                desk: all_zone_info_resource.my.desk,
                card: card_entity,
            });
        }
    }
}
