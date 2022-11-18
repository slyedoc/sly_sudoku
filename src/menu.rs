use bevy::{app::AppExit, prelude::*};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


use std::fmt::{Display, Formatter};

use crate::{
    ui::{FontAssets, Theme, ThemeToggle, ThickLine}, 
    events::*, AppState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(menu_button_system);
    }
}


#[derive(EnumIter, Debug, Component)]
enum MenuButton {
    New,
    Reset,
    Solve,
    ThemeToggle,
    Quit,
}

impl Display for MenuButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuButton::New => write!(f, "New Game"),
            MenuButton::Reset => write!(f, "Reset"),
            MenuButton::Solve => write!(f, "Solve"),
            MenuButton::ThemeToggle => write!(f, "Theme"),
            MenuButton::Quit => write!(f, "Quit"),
        }
    }
}

pub fn create_menu(parent: &mut ChildBuilder, theme: &Theme, font_assets: &FontAssets) {
    // Menu buttons
    parent
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(20.0),
                    right: Val::Px(20.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: theme.line_thick.into(),
            ..default()
        })
        .insert(ThickLine)
        .insert(Name::new("Menu Buttons"))
        .with_children(|parent| {
            for button in MenuButton::iter() {
                let name = format!("{button}");
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                                margin: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: theme.btn_normal.into(),
                            ..default()
                        },
                        Name::new(name.clone()),
                        button,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: name,
                                    style: TextStyle {
                                        font: font_assets.ui_font.clone(),
                                        font_size: 30.0,
                                        color: theme.text,
                                    },
                                }],
                                alignment: TextAlignment {
                                    vertical: VerticalAlign::Center,
                                    horizontal: HorizontalAlign::Center,
                                },
                            },
                            ..default()
                        });
                    });
            }
        });
}

fn menu_button_system(
    mut interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut reset_event: EventWriter<Reset>,
    mut solve_event: EventWriter<Solve>,
    mut theme_toggle_event: EventWriter<ThemeToggle>,
    mut exit_event: EventWriter<AppExit>,
    mut app_state: ResMut<State<AppState>>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match menu_button {
                MenuButton::New => {
                    app_state.set(AppState::Loading).unwrap();
                }
                MenuButton::Reset => {
                    reset_event.send(Reset);
                }
                MenuButton::Solve => {
                    solve_event.send(Solve);
                }
                MenuButton::ThemeToggle => {
                    theme_toggle_event.send(ThemeToggle);
                }
                MenuButton::Quit => {
                    exit_event.send(AppExit);
                }
            },
            _ => {}
        }
    }
}

