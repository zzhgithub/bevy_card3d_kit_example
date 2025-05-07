use crate::card_info::CardInfo;
use crate::card_info::card_enums::{Attr, CardType, Race};
use bevy::prelude::*;
use bevy_card3d_kit::prelude::Card;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;
use mlua::prelude::LuaUserDataFields;
use mlua::{UserData, Value};
use std::str::FromStr;

// 卡片信息的方法名称
pub const CARD_INFO_FUNC: &str = "get_card_info";

pub struct LuaPlugin;

impl Plugin for LuaPlugin {
    fn build(&self, app: &mut App) {
        app.add_scripting::<LuaRuntime>(|runtime| {
            // todo
        });
        app.add_systems(Startup, setup);
        app.add_systems(Update, add_card_info_by_script);
    }
}

impl UserData for CardInfo {}
fn setup(mut scripting_runtime: ResMut<LuaRuntime>) {
    scripting_runtime.with_engine_mut(|engine| {
        engine
            .register_userdata_type::<CardInfo>(|test| {
                test.add_field_method_get("id", |_, this| Ok(this.clone().id));
                test.add_field_method_get("name", |_, this| Ok(this.clone().name));
            })
            .unwrap();
        let test_constructor = engine
            .create_function(
                |_,
                 (id, name, card_type, attr, race, cost, ack): (
                    String,
                    String,
                    String,
                    String,
                    String,
                    usize,
                    u32,
                )| {
                    Ok(CardInfo {
                        id,
                        name,
                        card_type: CardType::from_str(card_type.as_str()).unwrap(),
                        attr: Attr::from_str(attr.as_str()).unwrap(),
                        race: Race::from_str(race.as_str()).unwrap(),
                        cost,
                        ack,
                    })
                },
            )
            .unwrap();
        engine.globals().set("CardInfo", test_constructor).unwrap();
    });
}

fn add_card_info_by_script(
    mut commands: Commands,
    mut scripted_entities: Query<(Entity, &mut LuaScriptData), (With<Card>, Added<LuaScriptData>)>,
    mut scripting_runtime: ResMut<LuaRuntime>,
) {
    for (entity, mut script_data) in &mut scripted_entities {
        if let Ok(lua_value) =
            scripting_runtime.call_fn_with_ns(CARD_INFO_FUNC, &mut script_data, entity, ())
        {
            scripting_runtime.with_engine_mut(|engine| {
                if let Ok(value) = engine.registry_value::<Value>(&lua_value.0) {
                    if let Value::UserData(data) = value {
                        if let Ok(card_info) = data.borrow::<CardInfo>() {
                            commands.entity(entity).insert(card_info.clone());
                        }
                    }
                }
            });
        }
    }
}
