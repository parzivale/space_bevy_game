use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(157., 159., 168.);
const HOVER_BUTTON: Color = Color::rgb(215., 218., 232.);
const CLICK_BUTTON: Color = Color::rgb(97., 99., 105.);


pub fn spawn_pause_menu(mut commands: Commands, asset_sever: Res<AssetServer>) {
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
