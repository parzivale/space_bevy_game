use crate::GameState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]

enum Action {
    ToggleMenu,
}

#[derive(Component)]
struct PauseMenu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_startup_system(setup_menu)
            .add_system(toggle_menu);
    }
}

fn setup_menu(mut commands: Commands) {
    commands
        .spawn_bundle(InputManagerBundle {
            action_state: ActionState::default(),
            input_map: InputMap::new([(KeyCode::Escape, Action::ToggleMenu)]),
        })
        .insert(PauseMenu)
        .insert_bundle(NodeBundle {
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section("Hi", TextStyle::default()),
                ..default()
            });
            parent.spawn_bundle(ButtonBundle{

                ..default()
            });
        });
}

fn toggle_menu(
    mut input: Query<&mut ActionState<Action>, With<PauseMenu>>,
    mut state: ResMut<State<GameState>>,
    mut menu: Query<&mut Visibility, With<PauseMenu>>,
    mut windows: ResMut<Windows>,
) {
    let mut actions_state = input.single_mut();
    let mut menu = menu.single_mut();
    let window = windows.get_primary_mut().unwrap();

    if actions_state.pressed(Action::ToggleMenu) && state.current() == &GameState::Unpaused {
        state.set(GameState::Paused).unwrap();
        actions_state.consume(Action::ToggleMenu);
        menu.is_visible = true;
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    } else if actions_state.pressed(Action::ToggleMenu) && state.current() != &GameState::Unpaused {
        state.set(GameState::Unpaused).unwrap();
        actions_state.consume(Action::ToggleMenu);
        menu.is_visible = false;
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
}
