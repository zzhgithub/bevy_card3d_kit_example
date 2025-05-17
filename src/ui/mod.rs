use crate::card_info::CardInfo;
use crate::zone_info::ZoneInfo;
use bevy::color;
use bevy::color::palettes::css::{GRAY, GREEN};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::system::IntoObserverSystem;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::hover::HoverMap;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::{Card, CardLine};
use bevy_card3d_kit::zone::Zone;
use bevy_card3d_kit::zone::desk_zone::DeskZone;
use std::sync::Arc;

// 被UI中选中
#[derive(Component, Clone, Debug)]
pub struct UIChose;

// 确认按钮组件
#[derive(Component, Clone, Debug)]
pub struct ConfirmButton {
    pub min: usize,
    pub max: usize,
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
        if num >= confirm_button.min && num <= confirm_button.max {
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

#[derive(Event, Clone)]
pub struct ShowDialogBox<T> {
    // 标题
    pub text: String,
    // 不同区域 Vec<DeskZone,Name>
    pub zone_list: Vec<Entity>,
    // 手卡区域 Optional
    pub hand_list: Vec<Entity>,
    // 选择的张数
    pub min: usize,
    pub max: usize,
    // 生成事件的回调
    pub callback: Arc<dyn Fn(Vec<Entity>, Vec<Entity>) -> T + Send + Sync + 'static>,
}

#[derive(Event, Clone, Debug)]
pub enum EnterEvent {
    Test,
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
    mut query_card: Query<&mut CardInfo>,
    asset_server: Res<AssetServer>,
) {
    for dialog_box in show_dialog.read() {
        let cb = (dialog_box.callback).clone();
        let text = format!(
            "{} With Cost: {} ",
            dialog_box.text.clone(),
            if dialog_box.min == dialog_box.max {
                dialog_box.min.to_string()
            } else {
                format!("min {} max {}", dialog_box.min, dialog_box.max)
            }
        );
        ui_dialog(
            &mut commands,
            text,
            |content_parent| {
                // TODO 显示全部
                scroll_list(content_parent, |p| {
                    spawn_card_list(p, "Zone1".to_string(), &asset_server);
                    spawn_card_list(p, "Zone2".to_string(), &asset_server);
                    spawn_card_list(p, "Zone3".to_string(), &asset_server);
                });
            },
            move |click: Trigger<Pointer<Click>>,
                  mut enter_events: EventWriter<EnterEvent>,
                  mut commands: Commands,
                  query_enable: Query<&ButtonEnable>,
                  dialog_show: Query<Entity, With<DialogShow>>| {
                // FIXME 添加控制 确认显示的逻辑
                if let Ok(_) = query_enable.get(click.target()) {
                    let event = (cb)(vec![], vec![]);
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
            dialog_box.min,
            dialog_box.max,
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
    min: usize,
    max: usize,
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
                                .insert(ConfirmButton { min, max });
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

// fn spawn_content(mut parent: &mut RelatedSpawnerCommands<ChildOf>) {}

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
) {
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
                    for x in vec!["NAAI-A-001", "S001-A-001"] {
                        let image = asset_server.load(format!("cards/{}.png", x));
                        pic_contents
                            .spawn((
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
                                Pickable {
                                    should_block_lower: false,
                                    ..default()
                                },
                            ))
                            .observe(
                                // TODO 这里要改成其他的！
                                |click: Trigger<Pointer<Click>>,
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
                                            .insert(UIChose);
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
