mod components;
mod systems;

use avian2d::prelude::*;
use bevy::prelude::*;

use systems::{event, startup, update};

const GRAVITY: f32 = 1000.0;
const PIPE_TIMER: f32 = 3.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, States, Default)]
enum GameState {
    #[default]
    Start,
    Playing,
    Dead,
    Clearing,
}

#[derive(Resource, Deref, DerefMut)]
struct SpawnTimer(Timer);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .init_state::<GameState>()
        .insert_resource(Gravity(Vec2::NEG_Y * GRAVITY))
        .insert_resource(SpawnTimer(Timer::from_seconds(
            PIPE_TIMER,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, (startup::spawn_bird, startup::spawn_camera))
        .add_systems(
            Update,
            (
                update::try_jump,
                update::try_spawn_pipe.run_if(in_state(GameState::Playing)),
                update::check_collisions.run_if(in_state(GameState::Playing)),
            ),
        )
        .add_systems(OnEnter(GameState::Playing), event::start)
        .add_systems(OnEnter(GameState::Dead), event::kill)
        .add_systems(OnEnter(GameState::Clearing), event::clear)
        .run();
}
