use::bevy::prelude::*;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.2,0.2,0.2)))
    .add_plugins(DefaultPlugins)
    .run();
}
