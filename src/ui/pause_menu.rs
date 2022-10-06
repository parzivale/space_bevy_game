use crate::GameState;
use bevy::{app::AppExit, prelude::*};
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum MenuActions {
    ToggleMenu,
    ExitGame,
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<MenuActions>::default())
            .add_startup_system(setup_menu)
            .add_system(toggle_menu)
            .add_system(exit_game);
    }
}

pub fn setup_menu(mut commands: Commands, asset_sever: Res<AssetServer>) {
    let font = asset_sever.load("fonts/FiraCode[wght].ttf");

    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn_bundle(InputManagerBundle {
            action_state: ActionState::default(),
            input_map: InputMap::new([(KeyCode::Escape, MenuActions::ToggleMenu)]),
        })
        .insert(PauseMenu)
        .insert_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .with_children(|parent| {
            let parent_entity = parent.parent_entity();
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    ..default()
                })
                .insert(ActionStateDriver {
                    action: MenuActions::ExitGame,
                    entity: parent_entity,
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Exit Game",
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn toggle_menu(
    mut input: Query<&mut ActionState<MenuActions>, With<PauseMenu>>,
    mut state: ResMut<State<GameState>>,
    mut menu: Query<&mut Visibility, With<PauseMenu>>,
    mut windows: ResMut<Windows>,
) {
    let mut actions_state = input.single_mut();
    let mut menu = menu.single_mut();
    let window = windows.get_primary_mut().unwrap();

    if actions_state.pressed(MenuActions::ToggleMenu) && state.current() == &GameState::Unpaused {
        state.set(GameState::Paused).unwrap();
        actions_state.consume(MenuActions::ToggleMenu);
        menu.is_visible = true;
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    } else if actions_state.pressed(MenuActions::ToggleMenu)
        && state.current() != &GameState::Unpaused
    {
        state.set(GameState::Unpaused).unwrap();
        actions_state.consume(MenuActions::ToggleMenu);
        menu.is_visible = false;
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
}

fn exit_game(
    mut event: EventWriter<AppExit>,
    input: Query<&ActionState<MenuActions>, With<PauseMenu>>,
) {
    let actions_state = input.single();
    if actions_state.pressed(MenuActions::ExitGame) {
        event.send(AppExit);
    }
}
