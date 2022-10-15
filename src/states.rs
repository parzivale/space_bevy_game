use bevy::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum AppState {
    MainMenu,
    Game,
    Paused,
}

#[derive(Component)]
pub struct AppStateComponent(pub AppState);

pub fn despawn_recursive_from_state(
    mut commands: Commands,
    query: Query<(Entity, &AppStateComponent)>,
    state: Res<State<AppState>>,
) {
    for (e, a) in query.iter() {
        if *state.current() == a.0 {
            commands.entity(e).despawn_recursive();
        }
    }
}
