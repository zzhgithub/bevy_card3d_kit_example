use crate::card_info::CardInfo;
use crate::card_info::card_enums::CardType;
use crate::hand_card::CardLineResource;
use crate::zone_info::AllZoneInfoResource;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;
use bevy_card3d_kit::highlight::Highlight;
use bevy_card3d_kit::prelude::card_state::CardState;
use bevy_card3d_kit::prelude::{Card, CardLine, HandCard, Moveable};
use bevy_card3d_kit::zone::desk_zone::{DeskCard, DeskZone, DeskZoneChangedEvent};
use bevy_scriptum::Script;
use bevy_scriptum::runtimes::lua::LuaScript;

pub const CAN_SET_COLOR: Srgba = bevy::color::palettes::css::LIGHT_SKY_BLUE;

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
                        padding: UiRect::all(Val::Px(2.0)),
                        row_gap: Val::Px(1.0),
                        column_gap: Val::Px(1.0),
                        ..Default::default()
                    },
                    BackgroundColor(bevy::color::palettes::css::DARK_GRAY.into()),
                ))
                .with_children(|parent| {
                    spawn_button(parent, "init desk".to_string(), on_click_init_desk);
                    spawn_button(parent, "draw".to_string(), on_click_draw);
                    spawn_button(parent, "highlight".to_string(), on_click_highlight);
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

fn on_click_highlight(
    _click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    all_zone_info_resource: Res<AllZoneInfoResource>,
    card_line_resource: Res<CardLineResource>,
    query_card_line: Query<&CardLine>,
    query_desks: Query<&DeskZone>,
    query_cards: Query<&CardInfo>,
) {
    if let Ok(lx_zone) = query_desks.get(all_zone_info_resource.my.lx) {
        let lx_used = lx_zone.card_list.len();
        let lx_remain = 6 - lx_used;
        if let Ok(card_line) = query_card_line.get(card_line_resource.my_card_line) {
            let hand_num = card_line.card_list.len();
            match_can_set(
                &card_line.card_list,
                &mut commands,
                hand_num,
                lx_remain,
                query_cards,
            );
            match_can_set(
                &lx_zone.card_list,
                &mut commands,
                hand_num,
                lx_remain,
                query_cards,
            );
        }
    }
}

fn match_can_set(
    list: &Vec<Entity>,
    commands: &mut Commands,
    hand_num: usize,
    lx_remain: usize,
    query_cards: Query<&CardInfo>,
) {
    for card_entity in list.iter() {
        if let Ok(card_info) = query_cards.get(*card_entity) {
            match card_info.card_type {
                CardType::Arcane => {
                    // do nothing
                }
                _ => {
                    if card_info.cost <= hand_num - 1 && card_info.cost <= lx_remain {
                        // 这样的卡才能设置！
                        commands.entity(*card_entity).insert(Highlight {
                            color: CAN_SET_COLOR.into(),
                        });
                    }
                }
            }
        }
    }
}
