mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod movement;
mod player_input;
mod random_move;

use crate::prelude::*;

use self::{
    collisions::collisions_system, end_turn::end_turn_system, entity_render::entity_render_system,
    map_render::map_render_system, movement::movement_system, player_input::player_input_system,
    random_move::random_move_system,
};

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement_system())
        .flush()
        .add_system(collisions_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move_system())
        .flush()
        .add_system(movement_system())
        .flush()
        .add_system(collisions_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(end_turn_system())
        .build()
}
