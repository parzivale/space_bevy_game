use bevy::prelude::*;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {

    let menu_style = Style{
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        justify_content: JustifyContent::Center,
        ..default()
    };

    let button_style = Style{
        ..default()
    };



    let field = | |{};

    commands.spawn_bundle(NodeBundle::default());
}