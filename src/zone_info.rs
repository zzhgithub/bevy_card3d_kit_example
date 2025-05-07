use bevy::prelude::*;
use bevy_card3d_kit::prelude::card_state::CardState;
use bevy_card3d_kit::zone::desk_zone::DeskZone;
use bevy_card3d_kit::zone::{Zone, ZoneMaterialGetter, bind_zone_render};
use std::f32::consts::PI;

/// 场地信息的定义

#[derive(Clone, Debug, Component)]
pub struct ZoneInfo {
    pub zone_type: ZoneType,
    pub opponent: bool,
}

#[derive(Clone, Debug)]
pub enum ZoneType {
    Nothing,
    // 战场
    BattleField,
    // 准备区
    PreparationField,
    // 安全屋
    SafeField,
    // 理性区
    LxField,
    // 激情区
    JqField,
    // 卡组
    DeskField,
    //墓地
    GraveField,
}

impl ZoneMaterialGetter for ZoneInfo {
    fn get_mal(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Handle<StandardMaterial> {
        match self.zone_type {
            ZoneType::Nothing => {
                return materials.add(Color::BLACK);
            }
            ZoneType::DeskField => {
                return materials.add(Color::WHITE);
            }
            ZoneType::GraveField => {
                return materials.add(Color::WHITE);
            }
            _ => {}
        };

        let path = match self.zone_type {
            ZoneType::BattleField => "stone_1",
            ZoneType::PreparationField => "stone_2",
            ZoneType::SafeField => "safe",
            ZoneType::LxField => "lx",
            ZoneType::JqField => "jq",
            _ => "",
        };
        materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load(format!("zones/{}.png", path))),
            unlit: true,
            ..Default::default()
        })
    }
}

pub struct ZoneInfoPlugin;

impl Plugin for ZoneInfoPlugin {
    fn build(&self, app: &mut App) {
        bind_zone_render::<ZoneInfo>(app);
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    render_all_zone(&mut commands, 4.0, 1.2);
}

fn render_all_zone(commands: &mut Commands, a: f32, mid: f32) {
    let half_a = a * 0.5;
    let half_mid = mid * 0.5;
    let mid_point = Transform::default();

    // 加载中心块
    commands.spawn((
        Name::new("Mid Zone"),
        Zone {
            center: mid_point.clone(),
            size: Vec2::new(a * 6.0 - 0.1, mid) - 0.1,
        },
        ZoneInfo {
            zone_type: ZoneType::Nothing,
            opponent: false,
        },
    ));
    // ==============对手===================
    //对方准备区
    let opponent_prepare = commands
        .spawn((
            Name::new("opponent Prepare Zone"),
            Zone {
                center: Transform::from_xyz(-3.0 * half_a, half_mid + half_a, 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::PreparationField,
                opponent: true,
            },
        ))
        .id();
    // 对方三个战场
    let opponent_battle1 = commands
        .spawn((
            Name::new("opponent Battle Zone 1"),
            Zone {
                center: Transform::from_xyz(-1.0 * half_a, half_mid + half_a, 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::BattleField,
                opponent: true,
            },
        ))
        .id();
    let opponent_battle2 = commands
        .spawn((
            Name::new("opponent Battle Zone 2"),
            Zone {
                center: Transform::from_xyz(half_a, half_mid + half_a, 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::BattleField,
                opponent: true,
            },
        ))
        .id();
    let opponent_battle3 = commands
        .spawn((
            Name::new("opponent Battle Zone 3"),
            Zone {
                center: Transform::from_xyz(3.0 * half_a, half_mid + half_a, 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::BattleField,
                opponent: true,
            },
        ))
        .id();
    //上方安全屋 x4
    let opponent_safe1 = commands
        .spawn((
            Name::new("opponent Safe Zone 1"),
            Zone {
                center: Transform::from_xyz(-3.0 * half_a, half_mid + half_a + a, 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::SafeField,
                opponent: true,
            },
        ))
        .id();
    let opponent_safe2 = commands
        .spawn((
            Name::new("opponent Safe Zone 2"),
            Zone {
                center: Transform::from_xyz(-1.0 * half_a, half_mid + half_a + a, 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::SafeField,
                opponent: true,
            },
        ))
        .id();
    let opponent_safe3 = commands
        .spawn((
            Name::new("opponent Safe Zone 3"),
            Zone {
                center: Transform::from_xyz(half_a, half_mid + half_a + a, 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::SafeField,
                opponent: true,
            },
        ))
        .id();
    let opponent_safe4 = commands
        .spawn((
            Name::new("opponent Safe Zone 4"),
            Zone {
                center: Transform::from_xyz(3.0 * half_a, half_mid + half_a + a, 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::SafeField,
                opponent: true,
            },
        ))
        .id();
    // 上方理性区
    let opponent_lx = commands
        .spawn((
            Name::new("opponent LX Zone"),
            Zone {
                center: Transform::from_xyz(-5.0 * half_a, half_mid + a, 0.0)
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, PI)),
                size: Vec2::new(a - 0.1, a * 2.0 - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::LxField,
                opponent: true,
            },
            DeskZone { card_list: vec![] },
            CardState {
                face_up: true,
                vertical: false,
            },
        ))
        .id();
    // 上方激情区
    let opponent_jq = commands
        .spawn((
            Name::new("opponent JQ Zone"),
            Zone {
                center: Transform::from_xyz(5.0 * half_a, half_mid + a, 0.0)
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, PI)),
                size: Vec2::new(a - 0.1, a * 2.0 - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::JqField,
                opponent: true,
            },
            CardState {
                face_up: false,
                vertical: false,
            },
        ))
        .id();

    // 上方卡组
    let opponent_desk = commands
        .spawn((
            Name::new("opponent Desk Zone"),
            DeskZone::default(),
            Zone {
                center: Transform::from_xyz(-7.0 * half_a, half_mid + half_a, 0.0)
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, PI)),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::DeskField,
                opponent: true,
            },
            CardState {
                face_up: false,
                vertical: true,
            },
        ))
        .id();
    // 上方墓地
    let opponent_grave = commands
        .spawn((
            Name::new("opponent GraveField Zone"),
            Zone {
                center: Transform::from_xyz(-7.0 * half_a, half_mid + half_a + a, 0.0)
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, PI)),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::GraveField,
                opponent: true,
            },
        ))
        .id();
    // ==============自己===================
    //准备区
    let prepare = commands
        .spawn((
            Name::new("Prepare Zone"),
            Zone {
                center: Transform::from_xyz(3.0 * half_a, -(half_mid + half_a), 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::PreparationField,
                opponent: false,
            },
        ))
        .id();
    // 三个战场
    let battle1 = commands
        .spawn((
            Name::new("Battle Zone 1"),
            Zone {
                center: Transform::from_xyz(-1.0 * half_a, -(half_mid + half_a), 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::BattleField,
                opponent: false,
            },
        ))
        .id();
    let battle2 = commands
        .spawn((
            Name::new("Battle Zone 2"),
            Zone {
                center: Transform::from_xyz(half_a, -(half_mid + half_a), 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::BattleField,
                opponent: false,
            },
        ))
        .id();
    let battle3 = commands
        .spawn((
            Name::new("Battle Zone 3"),
            Zone {
                center: Transform::from_xyz(-3.0 * half_a, -(half_mid + half_a), 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::BattleField,
                opponent: false,
            },
        ))
        .id();
    //安全屋 x4
    let safe1 = commands
        .spawn((
            Name::new("Safe Zone 1"),
            Zone {
                center: Transform::from_xyz(-3.0 * half_a, -(half_mid + half_a + a), 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::SafeField,
                opponent: false,
            },
        ))
        .id();
    let safe2 = commands
        .spawn((
            Name::new("Safe Zone 2"),
            Zone {
                center: Transform::from_xyz(-1.0 * half_a, -(half_mid + half_a + a), 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::SafeField,
                opponent: false,
            },
        ))
        .id();
    let safe3 = commands
        .spawn((
            Name::new("Safe Zone 3"),
            Zone {
                center: Transform::from_xyz(half_a, -(half_mid + half_a + a), 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::SafeField,
                opponent: false,
            },
        ))
        .id();
    let safe4 = commands
        .spawn((
            Name::new("Safe Zone 4"),
            Zone {
                center: Transform::from_xyz(3.0 * half_a, -(half_mid + half_a + a), 0.0),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::SafeField,
                opponent: false,
            },
        ))
        .id();
    // 理性区
    let lx = commands
        .spawn((
            Name::new("LX Zone"),
            Zone {
                center: Transform::from_xyz(5.0 * half_a, -(half_mid + a), 0.0),
                size: Vec2::new(a - 0.1, a * 2.0 - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::LxField,
                opponent: false,
            },
            DeskZone { card_list: vec![] },
            CardState {
                face_up: true,
                vertical: false,
            },
        ))
        .id();
    // 激情区
    let jq = commands
        .spawn((
            Name::new("JQ Zone"),
            Zone {
                center: Transform::from_xyz(-5.0 * half_a, -(half_mid + a), 0.0),
                size: Vec2::new(a - 0.1, a * 2.0 - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::JqField,
                opponent: false,
            },
            CardState {
                face_up: false,
                vertical: false,
            },
        ))
        .id();

    // 卡组
    let desk = commands
        .spawn((
            Name::new("Desk Zone"),
            DeskZone::default(),
            Zone {
                center: Transform::from_xyz(7.0 * half_a, -(half_mid + half_a), 0.0)
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, PI)),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::DeskField,
                opponent: true,
            },
            CardState {
                face_up: false,
                vertical: true,
            },
        ))
        .id();
    // 墓地
    let grave = commands
        .spawn((
            Name::new("GraveField Zone"),
            Zone {
                center: Transform::from_xyz(7.0 * half_a, -(half_mid + half_a + a), 0.0)
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, PI)),
                size: Vec2::new(a - 0.1, a - 0.1),
            },
            ZoneInfo {
                zone_type: ZoneType::GraveField,
                opponent: true,
            },
        ))
        .id();
    commands.insert_resource(AllZoneInfoResource {
        my: AllZoneInfo {
            desk: desk,
            grave: grave,
            lx: lx,
            jq: jq,
            battle1: battle1,
            battle2: battle2,
            battle3: battle3,
            prepare: prepare,
            safe1: safe1,
            safe2: safe2,
            safe3: safe3,
            safe4: safe4,
        },
        opponent: AllZoneInfo {
            desk: opponent_desk,
            grave: opponent_grave,
            lx: opponent_lx,
            jq: opponent_jq,
            battle1: opponent_battle1,
            battle2: opponent_battle2,
            battle3: opponent_battle3,
            prepare: opponent_prepare,
            safe1: opponent_safe1,
            safe2: opponent_safe2,
            safe3: opponent_safe3,
            safe4: opponent_safe4,
        },
    });
}

#[derive(Debug, Resource, Clone)]
pub struct AllZoneInfoResource {
    pub my: AllZoneInfo,
    pub opponent: AllZoneInfo,
}

#[derive(Debug, Clone)]
pub struct AllZoneInfo {
    pub desk: Entity,
    pub grave: Entity,
    pub lx: Entity,
    pub jq: Entity,
    pub battle1: Entity,
    pub battle2: Entity,
    pub battle3: Entity,
    pub prepare: Entity,
    pub safe1: Entity,
    pub safe2: Entity,
    pub safe3: Entity,
    pub safe4: Entity,
}
