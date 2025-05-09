use crate::card_info::CardInfo;
use crate::card_info::card_enums::CardType;
use crate::debug_lab::CNA_SET_ON_COLOR;
use bevy::prelude::*;
use bevy_card3d_kit::highlight::Highlight;

// 可以进行放置
#[derive(Component, Clone, Copy, Debug)]
pub struct CanSet;

// 可以放置到卡片上面
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CanSetOn(pub Vec<CardType>);

// 卡片设置到场地的插件
pub struct CardSetZonePlugin;

impl Plugin for CardSetZonePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CanSetOn>();
        app.add_observer(on_drag_start);
        app.add_observer(on_drag_end);
        // TODO 处理登场效果
    }
}

fn on_drag_start(
    drag_start: Trigger<Pointer<DragStart>>,
    query: Query<&CardInfo, With<CanSet>>,
    query_zone: Query<(Entity, &CanSetOn), Without<CanSet>>,
    mut commands: Commands,
) {
    if let Ok(card_info) = query.get(drag_start.target()) {
        for (entity, can_set_on) in query_zone.iter() {
            if can_set_on.0.contains(&card_info.card_type) {
                commands.entity(entity).insert(Highlight {
                    color: CNA_SET_ON_COLOR.into(),
                });
            }
        }
    }
}

fn on_drag_end(
    drag_end: Trigger<Pointer<DragEnd>>,
    query: Query<&CardInfo, With<CanSet>>,
    query_zone: Query<(Entity, &CanSetOn), Without<CanSet>>,
    mut commands: Commands,
) {
    if let Ok(card_info) = query.get(drag_end.target()) {
        for (entity, can_set_on) in query_zone.iter() {
            if can_set_on.0.contains(&card_info.card_type) {
                commands.entity(entity).remove::<Highlight>();
            }
        }
    }
}
