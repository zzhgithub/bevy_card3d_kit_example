use bevy::prelude::*;
use bevy_card3d_kit::prelude::card_state::CardState;
use bevy_card3d_kit::prelude::{CardLine, HAND_CARD_LEVEL};

pub struct HandCardPlugin;

impl Plugin for HandCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
fn setup(mut commands: Commands) {
    let card_line_entity = commands
        .spawn((
            CardLine {
                transform: Transform::from_xyz(0.0, -6.7, HAND_CARD_LEVEL),
                card_list: vec![],
            },
            CardState {
                face_up: true,
                vertical: true,
            },
        ))
        .id();
    let opponent_card_line_entity = commands
        .spawn((
            CardLine {
                transform: Transform::from_xyz(0.0, 6.7, HAND_CARD_LEVEL),
                card_list: vec![],
            },
            CardState {
                face_up: false,
                vertical: true,
            },
        ))
        .id();

    commands.insert_resource(CardLineResource {
        my_card_line: card_line_entity,
        opponent_card_line: opponent_card_line_entity,
    });
}

#[derive(Resource, Clone, Debug)]
pub struct CardLineResource {
    pub my_card_line: Entity,
    pub opponent_card_line: Entity,
}
