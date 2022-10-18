use bevy::prelude::*;

#[derive(Bundle)]
struct field{
    label: &str,
    #[bundle]
    root: NodeBundle,
    #[bundle]
    button: Option<ButtonBundle>,
    #[bundle]
    text: Option<TextBundle>,
}

#[derive(Bundle)]
pub struct PauseMenuBundle{
    #[bundle]
    root: NodeBundle,
    #[bundle]
    children: Vec<feild>,
}

impl PauseMenuBundle{
    fn new() -> Self {
        Self {
            root: Default::default(),
            children: Vec::new(),
        }
    }
}

impl Default for PauseMenuBundle{
    fn default() -> Self {
        Self::new()
    }}