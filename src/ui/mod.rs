use crate::card_info::CardInfo;
use crate::zone_info::ZoneInfo;
use bevy::color;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::{Card, CardLine};
use bevy_card3d_kit::zone::Zone;
use bevy_card3d_kit::zone::desk_zone::DeskZone;
use std::sync::Arc;

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
        app.add_systems(Update, show_dialog);
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
) {
    for dialog_box in show_dialog.read() {
        let cb = (dialog_box.callback).clone();

        ui_dialog(
            &mut commands,
            dialog_box.text.clone(),
            |_| {
                // TODO 显示全部
            },
            move |_click: Trigger<Pointer<Click>>,
                  mut enter_events: EventWriter<EnterEvent>,
                  mut commands: Commands,
                  dialog_show: Query<Entity, With<DialogShow>>| {
                // FIXME 添加控制 确认显示的逻辑
                let event = (cb)(vec![], vec![]);
                info!("Sending {:?}", event);
                // enter_events.write(event);
                if let Ok(single) = dialog_show.single() {
                    commands.entity(single).despawn();
                }
            },
            move |_click: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  dialog_show: Query<Entity, With<DialogShow>>| {
                if let Ok(single) = dialog_show.single() {
                    commands.entity(single).despawn();
                }
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
                            spawn_button(
                                dialog_button,
                                "Confirm".to_string(),
                                color::palettes::css::GREEN.into(),
                                observer_confirm,
                            );
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
) {
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
        .observe(observer);
}
