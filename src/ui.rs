use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

use crate::board::{Cell};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontAssets>()
        .init_resource::<ButtonAssets>()
        .add_plugin(InspectorPlugin::<ButtonAssets>::new())
        .add_system(button_system);
    }
}

#[derive(Inspectable)]
pub struct ButtonAssets {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
    pub selected: Color,
}

impl Default for ButtonAssets {
    fn default() -> Self {
        Self {
            normal: Color::WHITE,
            hovered: Color::GRAY,
            pressed: Color::DARK_GRAY,
            selected: Color::GREEN,
        }
    }
}

pub struct FontAssets {
    pub font_color: Color,
    pub ui_font: Handle<Font>,
}

impl FromWorld for FontAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let ui_font = asset_server.load("fonts/FiraSans-Bold.ttf");
        Self {
            font_color: Color::BLACK,
            ui_font,
        }
    }
}

impl FontAssets {
    #[allow(dead_code)]
    pub fn h1(&self, text: String, color: Color) -> TextSection {
        TextSection {
            value: text,
            style: TextStyle {
                font: self.ui_font.clone(),
                font_size: 30.0,
                color,
            },
        }
    }

    #[allow(dead_code)]
    pub fn title(&self, text: String, color: Color) -> TextSection {
        TextSection {
            value: text,
            style: TextStyle {
                font: self.ui_font.clone(),
                font_size: 90.0,
                color,
            },
        }
    }

    #[allow(dead_code)]
    pub fn sub_title(&self, text: String, color: Color) -> TextSection {
        TextSection {
            value: text,
            style: TextStyle {
                font: self.ui_font.clone(),
                font_size: 16.0,
                color,
            },
        }
    }

    #[allow(dead_code)]
    pub fn cell(&self, text: impl Into<String>) -> Text {
        Text {
            sections: vec![TextSection {
                value: text.into(),
                style: TextStyle {
                    font: self.ui_font.clone(),
                    font_size: 90.0,
                    color: self.font_color,
                },
            }],
            ..default()
        }
    }
}


// Note: exclude the cell buttons in the query, look for better way to do this
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),        
        (Changed<Interaction>, With<Button>, Without<Cell>),
    >,
    
   
    button_assets: Res<ButtonAssets>,
    
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                //text.sections[0].value = "Press".to_string();
                *color = button_assets.pressed.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                *color = button_assets.hovered.into();
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                *color = button_assets.normal.into();
            }
        }
    }
}
