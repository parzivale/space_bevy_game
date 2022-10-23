use bevy::prelude::*;

pub struct UiAssets{
    pub font: Handle<Font>,
}

impl FromWorld for UiAssets{
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource::<AssetServer>().unwrap();
        Self {
            font: assets.load("fonts/FiraCode[wght].ttf")
        }
    }
}

impl UiAssets{
    pub fn text_bundle(&self, content: &str, font_size: f32) -> TextBundle {
        let color = Color::ANTIQUE_WHITE;
        let style = TextStyle { color, font: self.font.clone(), font_size };
        TextBundle::from_section(content, style)
    }
}