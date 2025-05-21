use crate::card_info::CardInfo;
use crate::zone_info::ZoneInfo;
use bevy::color;
use bevy::color::palettes::css::{GRAY, GREEN};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::system::IntoObserverSystem;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::hover::HoverMap;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::card_state::CardState;
use bevy_card3d_kit::prelude::{Card, CardLine};
use bevy_card3d_kit::preview_plugins::ImagePreview;
use bevy_card3d_kit::zone::Zone;
use bevy_card3d_kit::zone::desk_zone::DeskZone;
use std::cmp::PartialEq;
use std::sync::Arc;

// 被UI中选中
#[derive(Component, Clone, Debug)]
pub struct UIChose(pub u32);

// 确认按钮组件
#[derive(Component, Clone, Debug)]
pub struct ConfirmButton {
    pub min: usize,
    pub max: usize,
    pub list: Vec<ZoneAndLimit>,
}

#[derive(Component, Clone, Debug)]
pub struct ButtonEnable;

fn confirm_button_color_system(
    mut commands: Commands,
    button_query: Query<(Entity, &ConfirmButton)>,
    ui_chose_query: Query<&UIChose>,
) {
    if let Ok((button, confirm_button)) = button_query.single() {
        let num = ui_chose_query.iter().len();
        let mut check_a = true;
        for zone_and_limit in confirm_button.list.iter() {
            let index = zone_and_limit.entity.index();
            let i = ui_chose_query.iter().filter(|&x| x.0 == index).count();
            if i >= zone_and_limit.min && i <= zone_and_limit.max {
                // do nothing
            } else {
                check_a = false;
                break;
            }
        }
        if num >= confirm_button.min && num <= confirm_button.max && check_a {
            commands
                .entity(button)
                .insert(ButtonEnable)
                .insert(BackgroundColor(GREEN.into()));
        } else {
            commands
                .entity(button)
                .remove::<ButtonEnable>()
                .insert(BackgroundColor(GRAY.into()));
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UICardType {
    Hand,
    Zone,
}

#[derive(Component, Clone, Debug)]
pub struct UICardInfo {
    pub card_type: UICardType,
    pub card_info: CardInfo,
    pub card_state: CardState,
    pub zone_card_pair: ZoneCardPair,
}

// 位置卡片对
pub type ZoneCardPair = (Entity, Entity);

#[derive(Clone, Debug)]
pub struct ZoneAndLimit {
    pub entity: Entity,
    pub min: usize,
    pub max: usize,
}

#[derive(Event, Clone)]
pub struct ShowDialogBox<T> {
    pub card: Entity,
    // 标题
    pub text: String,
    // 不同区域 Vec<DeskZone,Name>
    pub zone_list: Vec<ZoneAndLimit>,
    // 手卡区域 Optional
    pub hand_list: Vec<ZoneAndLimit>,
    // 选择的张数
    pub min: usize,
    pub max: usize,
    // 生成事件的回调
    pub callback:
        Arc<dyn Fn(Entity, Vec<ZoneCardPair>, Vec<ZoneCardPair>) -> T + Send + Sync + 'static>,
}

#[derive(Event, Clone, Debug)]
pub enum EnterEvent {
    Test,
    SetCard {
        card: Entity,
        cost_hand: Vec<ZoneCardPair>,
        cost_jq: Vec<ZoneCardPair>,
    },
}

#[derive(Component, Clone, Debug)]
pub struct DialogShow;

pub struct ShowDialogPlugin;

impl Plugin for ShowDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShowDialogBox<EnterEvent>>();
        app.add_event::<EnterEvent>();
        app.add_systems(
            Update,
            (
                show_dialog,
                update_scroll_position,
                confirm_button_color_system,
            ),
        );
    }
}

// 显示对话框
fn show_dialog(
    mut commands: Commands,
    mut show_dialog: EventReader<ShowDialogBox<EnterEvent>>,
    // 查询位置
    mut query_zone: Query<(&ZoneInfo, &Name, &DeskZone)>,
    // 查询手卡
    mut query_card_line: Query<&mut CardLine>,
    // 查询卡片信息
    mut query_card: Query<(&CardInfo, &CardState)>,
    asset_server: Res<AssetServer>,
) {
    for dialog_box in show_dialog.read() {
        let box_card_entity = dialog_box.card.clone();
        // 复制回调
        let cb = (dialog_box.callback).clone();

        let all_list = dialog_box
            .hand_list
            .iter()
            .chain(dialog_box.zone_list.iter())
            .cloned()
            .collect();

        let mut hand_list = vec![];
        let mut card_list = vec![];
        // 计算Zone和卡片
        for zone_and_limit in dialog_box.hand_list.iter() {
            if let Ok(card_line) = query_card_line.get(zone_and_limit.entity.clone()) {
                let mut info_list = vec![];
                for card_entity in card_line.card_list.iter() {
                    if let Ok((card_info, card_state)) = query_card.get(*card_entity) {
                        let ui_card_info = UICardInfo {
                            card_type: UICardType::Hand,
                            card_info: card_info.clone(),
                            card_state: card_state.clone(),
                            zone_card_pair: (zone_and_limit.clone().entity, card_entity.clone()),
                        };
                        if *card_entity != box_card_entity {
                            info_list.push(ui_card_info);
                        }
                    }
                }
                if info_list.len() > 0 {
                    hand_list.push(("Hand Card", zone_and_limit, info_list));
                }
            }
        }

        for zone_and_limit in dialog_box.zone_list.iter() {
            if let Ok((zone_info, name, desk_zone)) = query_zone.get(zone_and_limit.entity.clone())
            {
                let mut info_list = vec![];
                for card_entity in desk_zone.card_list.iter() {
                    if let Ok((card_info, card_state)) = query_card.get(*card_entity) {
                        let ui_card_info = UICardInfo {
                            card_type: UICardType::Zone,
                            card_info: card_info.clone(),
                            card_state: card_state.clone(),
                            zone_card_pair: (zone_and_limit.clone().entity, card_entity.clone()),
                        };
                        info_list.push(ui_card_info);
                    }
                }
                if info_list.len() > 0 {
                    card_list.push((name.to_string(), zone_and_limit, info_list));
                }
            }
        }

        ui_dialog(
            &mut commands,
            dialog_box.text.clone(),
            |content_parent| {
                // 显示全部
                scroll_list(content_parent, |p| {
                    for (name, zone_and_limit, list) in hand_list.clone() {
                        spawn_card_list(
                            p,
                            name.to_string(),
                            &asset_server,
                            list.clone(),
                            zone_and_limit.clone(),
                        );
                    }
                    for (name, zone_and_limit, list) in card_list.clone() {
                        spawn_card_list(
                            p,
                            name.clone(),
                            &asset_server,
                            list.clone(),
                            zone_and_limit.clone(),
                        );
                    }
                });
            },
            move |click: Trigger<Pointer<Click>>,
                  mut enter_events: EventWriter<EnterEvent>,
                  mut commands: Commands,
                  query_enable: Query<&ButtonEnable>,
                  dialog_show: Query<Entity, With<DialogShow>>,
                  query_chose: Query<(&UIChose, &UICardInfo)>| {
                // FIXME 添加控制 确认显示的逻辑
                if let Ok(_) = query_enable.get(click.target()) {
                    let hand = query_chose
                        .iter()
                        .filter(|&(_, info)| info.card_type == UICardType::Hand)
                        .map(|(_, info)| info.zone_card_pair)
                        .collect();
                    let zone = query_chose
                        .iter()
                        .filter(|&(_, info)| info.card_type == UICardType::Zone)
                        .map(|(_, info)| info.zone_card_pair)
                        .collect();
                    let event = (cb)(box_card_entity, hand, zone);
                    info!("Sending {:?}", event);
                    // enter_events.write(event);
                    if let Ok(single) = dialog_show.single() {
                        commands.entity(single).despawn();
                    }
                }
            },
            move |_click: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  dialog_show: Query<Entity, With<DialogShow>>| {
                if let Ok(single) = dialog_show.single() {
                    commands.entity(single).despawn();
                }
            },
            ConfirmButton {
                min: dialog_box.min,
                max: dialog_box.max,
                list: all_list,
            },
        );
    }
}

fn ui_dialog<E1: Event, B1: Bundle, M1, E2: Event, B2: Bundle, M2>(
    commands: &mut Commands,
    // 标题
    title: String,
    // 中心内容
    content: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
    // 确认
    observer_confirm: impl IntoObserverSystem<E1, B1, M1>,
    // 取消
    observer_cancel: impl IntoObserverSystem<E2, B2, M2>,
    confirm_button: ConfirmButton,
) {
    commands
        .spawn((
            DialogShow,
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(80.),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Stretch,
                        padding: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },
                    BackgroundColor(color::palettes::css::LIGHT_SKY_BLUE.with_alpha(0.5).into()),
                    Outline {
                        width: Val::Px(6.0),
                        offset: Default::default(),
                        color: color::palettes::css::DARK_BLUE.into(),
                    },
                ))
                .with_children(|dialog| {
                    // 标题
                    dialog
                        .spawn((
                            Name::new("Title"),
                            Node {
                                height: Val::Px(80.0),
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BackgroundColor(
                                color::palettes::css::LIGHT_SKY_BLUE.with_alpha(0.5).into(),
                            ),
                        ))
                        .with_children(|dialog_title| {
                            dialog_title.spawn((
                                Text::new(title),
                                TextFont {
                                    font: default(),
                                    font_size: 33.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });
                    // 中间的内容
                    content(dialog);
                    // 结尾按钮
                    dialog
                        .spawn((
                            Name::new("Button"),
                            Node {
                                height: Val::Px(80.0),
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            BackgroundColor(color::palettes::css::DARK_BLUE.with_alpha(0.5).into()),
                        ))
                        .with_children(|dialog_button| {
                            let entity = spawn_button(
                                dialog_button,
                                "Confirm".to_string(),
                                color::palettes::css::GREEN.into(),
                                observer_confirm,
                            );
                            dialog_button
                                .commands()
                                .entity(entity)
                                .insert(confirm_button);
                            spawn_button(
                                dialog_button,
                                "Cancel".to_string(),
                                color::palettes::css::RED.into(),
                                observer_cancel,
                            );
                        });
                });
        });
}

// 渲染按钮
fn spawn_button<E: Event, B: Bundle, M>(
    mut parent: &mut RelatedSpawnerCommands<ChildOf>,
    text: String,
    color: Color,
    observer: impl IntoObserverSystem<E, B, M>,
) -> Entity {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(80.),
                height: Val::Px(40.),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center, // 按钮文字居中
                align_items: AlignItems::Center,         // 垂直居中
                border: UiRect::all(Val::Px(2.0)),       // 边框
                ..Default::default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(color),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    // font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        })
        .observe(observer)
        .id()
}

fn spawn_card_list(
    mut parent: &mut RelatedSpawnerCommands<ChildOf>,
    title: String,
    asset_server: &Res<AssetServer>,
    list: Vec<UICardInfo>,
    zone_and_limit: ZoneAndLimit,
) {
    let entity_index = zone_and_limit.clone().entity.index();
    parent
        .spawn((
            Node {
                width: Val::Auto,
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Pickable::IGNORE,
            BackgroundColor(color::palettes::css::LIGHT_SKY_BLUE.with_alpha(0.5).into()),
        ))
        .with_children(|dialog| {
            // 标题
            dialog
                .spawn((
                    Name::new("Title"),
                    Node {
                        height: Val::Px(40.0),
                        width: Val::Auto,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(color::palettes::css::LIGHT_SKY_BLUE.with_alpha(0.5).into()),
                    Outline {
                        width: Val::Px(1.0),
                        offset: Default::default(),
                        color: color::palettes::css::WHITE.into(),
                    },
                    Pickable {
                        should_block_lower: false,
                        ..default()
                    },
                ))
                .with_children(|dialog_title| {
                    dialog_title.spawn((
                        Text::new(title),
                        TextFont {
                            font: default(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });
            dialog
                .spawn((
                    Name::new("picture"),
                    Node {
                        height: Val::Px(300.0),
                        width: Val::Auto,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        padding: UiRect::all(Val::Px(10.0)),
                        row_gap: Val::Px(10.0),
                        column_gap: Val::Px(15.0),
                        ..Default::default()
                    },
                    BackgroundColor(color::palettes::css::LIGHT_SKY_BLUE.with_alpha(0.5).into()),
                    Outline {
                        width: Val::Px(1.0),
                        offset: Default::default(),
                        color: color::palettes::css::WHITE.into(),
                    },
                    Pickable {
                        should_block_lower: false,
                        ..default()
                    },
                ))
                .with_children(|pic_contents| {
                    // FIXME: 测试代码 后面改成其他的 这里除了图片还 要知道属于的zone或者cardline!
                    for ui_card_info in list.iter() {
                        let image = if ui_card_info.card_state.face_up {
                            asset_server.load(format!("cards/{}.png", ui_card_info.card_info.id))
                        } else {
                            asset_server.load(format!("cards/{}.png", "back"))
                        };

                        pic_contents
                            .spawn((
                                ui_card_info.clone(),
                                Node {
                                    height: Val::Percent(100.0),
                                    width: Val::Auto,
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                ImageNode {
                                    image: image.clone(),
                                    ..default()
                                },
                                // 预览
                                Pickable {
                                    should_block_lower: false,
                                    ..default()
                                },
                            ))
                            .observe(
                                move |click: Trigger<Pointer<Click>>,
                                      mut commands: Commands,
                                      query: Query<&Outline>| {
                                    if let Ok(en) = query.get(click.target()) {
                                        commands
                                            .entity(click.target())
                                            .remove::<Outline>()
                                            .remove::<UIChose>();
                                    } else {
                                        commands
                                            .entity(click.target())
                                            .insert(Outline {
                                                width: Val::Px(5.0),
                                                offset: Val::Px(0.2),
                                                color: color::palettes::css::RED.into(),
                                            })
                                            .insert(UIChose(entity_index));
                                    }
                                },
                            );
                    }
                });
        });
}

pub fn scroll_list<F>(parent: &mut RelatedSpawnerCommands<ChildOf>, mut callback: F)
where
    F: FnMut(&mut RelatedSpawnerCommands<ChildOf>),
{
    parent
        .spawn((
            Node {
                display: Display::Flex,
                width: Val::Auto,
                height: Val::Auto,
                flex_direction: FlexDirection::Row,
                margin: UiRect::all(Val::Px(5.)),
                overflow: Overflow::scroll_x(),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            callback(parent);
        });
}

pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (mouse_wheel_event.x * 5.0, mouse_wheel_event.y * 5.0),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };
        // 换x和y轴
        std::mem::swap(&mut dx, &mut dy);
        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}
