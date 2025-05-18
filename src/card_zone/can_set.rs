use crate::card_info::CardInfo;
use crate::card_info::card_enums::CardType;
use crate::debug_lab::CNA_SET_ON_COLOR;
use crate::hand_card::CardLineResource;
use crate::ui::{EnterEvent, ShowDialogBox};
use crate::zone_info::AllZoneInfoResource;
use bevy::prelude::*;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy_card3d_kit::highlight::Highlight;
use bevy_card3d_kit::zone::desk_zone::DeskZone;
use bevy_card3d_kit::zone::events::CardOnZone;
use std::sync::Arc;

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
        app.add_observer(card_on_zone);
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

// 卡片在zone上的代码
fn card_on_zone(
    card_on_zone: Trigger<CardOnZone>,
    mut commands: Commands,
    query_zone: Query<(&CanSetOn, &DeskZone)>,
    query_card: Query<&CardInfo, With<CanSet>>,
    mut show_dialog: EventWriter<ShowDialogBox<EnterEvent>>,
    all_zone_info_resource: Res<AllZoneInfoResource>,
    card_line_resource: Res<CardLineResource>,
) {
    // TODO 这里要进行复杂的登场计算
    // 1.当前位置可以登场
    // 需要查看要登场时支付费用的内容
    // 发送要登场的事件

    if let Ok(card_info) = query_card.get(card_on_zone.card) {
        if let Ok((can_set_on, desk_zone)) = query_zone.get(card_on_zone.zone) {
            //TODO 检查当前费用是否足够!
            if can_set_on.0.contains(&card_info.card_type) {
                if card_info.card_type == CardType::Actor && desk_zone.card_list.len() > 0 {
                    return;
                }
                // 这里 还要处理模因卡的问题
                show_dialog.write(ShowDialogBox {
                    card: card_on_zone.card.clone(),
                    text: format!("Set Card {} ", card_info.name),
                    zone_list: vec![all_zone_info_resource.my.jq],
                    hand_list: vec![card_line_resource.my_card_line],
                    min: card_info.cost,
                    max: card_info.cost,
                    callback: Arc::new(|_, _| EnterEvent::Test),
                });
            }
        }
    }
}
