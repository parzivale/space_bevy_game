use crate::GameState;
use bevy::{app::AppExit, prelude::*};
use leafwing_input_manager::prelude::*;


pub fn exit_game(
    mut event: EventWriter<AppExit>,
    input: Query<&ActionState<MenuActions>, With<PauseMenu>>,
) {
    let actions_state = input.single();
    if actions_state.pressed(MenuActions::ExitGame) {
        event.send(AppExit);
    }
}


pub fn toggle_menu(
    mut input: Query<&mut ActionState<resources::PauseMenuActions>, With<PauseMenu>>,
    mut state: ResMut<State<GameState>>,
    mut menu: Query<&mut Visibility, With<PauseMenu>>,
    mut windows: ResMut<Windows>,
) {
    let mut actions_state = input.single_mut();
    let mut menu = menu.single_mut();
    let window = windows.get_primary_mut().unwrap();

    if actions_state.pressed(MenuActions::ToggleMenu) && state.current() == &GameState::Game {
        state.set(GameState::Paused).unwrap();
        actions_state.consume(MenuActions::ToggleMenu);
        menu.is_visible = true;
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    } else if actions_state.pressed(MenuActions::ToggleMenu) && state.current() != &GameState::Game
    {
        state.set(GameState::Game).unwrap();
        actions_state.consume(MenuActions::ToggleMenu);
        menu.is_visible = false;
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
}
