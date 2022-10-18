use bevy::prelude::*;



pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin{
    fn build(&self, app: &mut App) {
    }
}

fn setup_styles(){
    let menu_style = Style{
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        size: Size::new(Val::Auto,Val::Auto),
        ..default()
    };

    let resume_button_style = Style{
        aspect_ratio: Some(32./9.),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };
/*
    let resume_button_text_style = TextStyle {
    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    font_size: 40.0,
    color: Color::rgb(0.9, 0.9, 0.9),
    };
*/
}