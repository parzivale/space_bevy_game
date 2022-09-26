use crate::GameState;
use bevy::{app::AppExit, input::keyboard::KeyboardInput, prelude::*};
pub struct ActionsPlugin;

mod mouse;
mod movement;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Unpaused).with_system(movement::movement_input),
        )
        .add_system_set(SystemSet::on_update(GameState::Unpaused).with_system(mouse::mouse_look))
        .add_system(key_match);
    }
}

fn key_match(
    mut key_evr: EventReader<KeyboardInput>,
    mut exit: EventWriter<AppExit>,
    state: Res<State<GameState>>,
) {
    use bevy::input::ButtonState;
    for ev in key_evr.iter() {
        let keycode;
        if ev.key_code.is_none() {
            continue;
        } else {
            keycode = ev.key_code.unwrap();
        }

        match ev.state {
            ButtonState::Pressed => match keycode {
                KeyCode::Escape => match state.current() {
                    GameState::Unpaused => {
                        exit.send(AppExit);
                    }
                    _ => {}
                },
                _ => {}
            },
            ButtonState::Released => {
                let keycode = ev.key_code.expect("keycode is none");
                match keycode {
                    _ => {}
                }
            }
        }
    }
}
