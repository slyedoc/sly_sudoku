use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_tweening::{lens::*, *};

use crate::{board::Cell, lens::{Camera2dClearColorLens, UiColorLens}};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ThemeToggle>()
            .init_resource::<FontAssets>()
            .init_resource::<ThemeMode>()            
            .init_resource::<Theme>()
            .add_system(button_system)
            .add_system(theme_toggle_events);
    }
}

#[derive(Component)]
pub struct ThinLine;
#[derive(Component)]
pub struct ThickLine;

pub fn theme_toggle_events(
    mut commands: Commands,
    mut theme_toggle: EventReader<ThemeToggle>,
    mut theme_mode: ResMut<ThemeMode>,
    mut theme: ResMut<Theme>,
    button_query: Query<Entity, (With<Button>, With<UiColor>)>,
    text_query: Query<Entity, With<Text>>,
    thick_line_query: Query<Entity, (With<UiColor>, With<ThickLine>)>,
    thin_line_query: Query<Entity, (With<UiColor>, With<ThinLine>)>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    for _ in theme_toggle.iter() {    

        let old_theme = theme.clone();

        *theme_mode = match *theme_mode {
            ThemeMode::Light => ThemeMode::Dark,            
            ThemeMode::Dark => ThemeMode::Light,
        };

        let ease_fn = EaseFunction::CircularInOut;
        let tweening_type = TweeningType::Once;
        let duration = Duration::from_secs_f32(0.5);

        *theme = theme_mode.theme().clone();
        // update colors
        for e in button_query.iter() {
            commands.entity(e).insert(Animator::new(Tween::new(            
                ease_fn,
                tweening_type,
                duration,
                UiColorLens {
                    start: old_theme.btn_normal,
                    end: theme.btn_normal,
                },
            )));  
        }

        for e in thin_line_query.iter() {            
            commands.entity(e).insert(Animator::new(Tween::new(            
                ease_fn,
                tweening_type,
                duration,
                UiColorLens {
                    start: old_theme.line_thin,
                    end: theme.line_thin,
                },
            )));   
        }

        for e in thick_line_query.iter() {
            commands.entity(e).insert(Animator::new(Tween::new(            
                ease_fn,
                tweening_type,
                duration,
                UiColorLens {
                    start: old_theme.line_thick,
                    end: theme.line_thick,
                },
            )));   
        }

        for e in text_query.iter() {            
            commands.entity(e).insert(Animator::new(Tween::new(            
                ease_fn,
                tweening_type,
                duration,
                TextColorLens {
                    start: old_theme.btn_text,
                    end: theme.btn_text,
                    section: 0,
                },
            )));            
        }

        for e in camera_query.iter() {            
            commands.entity(e).insert(Animator::new(Tween::new(
                ease_fn,
                tweening_type,
                duration,
                Camera2dClearColorLens {
                    start: old_theme.background,
                    end: theme.background,
                },
            )));
        }
    }
}

/// Event to toggle the theme
pub struct ThemeToggle;

#[derive(Inspectable, Clone)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Light
    }
}

impl ThemeMode {
    pub fn theme(&self) -> &Theme {
        match self {
            ThemeMode::Light => &Theme::LIGHT,
            ThemeMode::Dark => &Theme::DARK,
        }
    }
}

#[derive(Inspectable, Clone)]
pub struct Theme {
    pub btn_text: Color,
    pub btn_normal: Color,
    pub btn_hovered: Color,
    pub btn_pressed: Color,
    pub btn_selected: Color,
    pub line_thin: Color,
    pub line_thick: Color,
    pub background: Color,
}

impl Default for Theme {
    fn default() -> Self {
        ThemeMode::default().theme().clone()
    }
}

impl Theme {

    const LIGHT: Theme = Theme {
        btn_text: Color::BLACK,
        btn_normal: Color::WHITE,
        btn_hovered: Color::GRAY,
        btn_pressed: Color::DARK_GRAY,
        btn_selected: Color::GREEN,        
        line_thin: Color::GRAY,
        line_thick: Color::BLACK,        
        background: Color::WHITE,
    };

    const DARK: Theme = Theme {
        btn_text: Color::WHITE,
        btn_normal: Color::BLACK,
        btn_hovered: Color::GRAY,
        btn_pressed: Color::DARK_GRAY,
        btn_selected: Color::GREEN,
        line_thin: Color::GRAY,
        line_thick: Color::WHITE,
        background: Color::BLACK,
    };
}

pub struct FontAssets {
    pub ui_font: Handle<Font>,
}

impl FromWorld for FontAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let ui_font = asset_server.load("fonts/FiraSans-Bold.ttf");
        Self {
            ui_font,
        }
    }
}

impl FontAssets {

    #[allow(dead_code)]
    pub fn btn(&self, text: impl Into<String>, theme: &Theme) -> Text {
        Text {
            sections: vec![TextSection {
                value: text.into(),
                style: TextStyle {
                    font: self.ui_font.clone(),
                    font_size: 90.0,
                    color: theme.btn_text,
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }
        }
    }
}

// Note: exclude the cell buttons in the query, look for better way to do this
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, Without<Cell>),
    >,

    theme: Res<Theme>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                //text.sections[0].value = "Press".to_string();
                *color = theme.btn_pressed.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                *color = theme.btn_hovered.into();
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                *color = theme.btn_normal.into();
            }
        }
    }
}
