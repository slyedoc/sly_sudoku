use bevy::{prelude::*, app::AppExit};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::fmt::{Display, Formatter};

use crate::{ui::{ButtonAssets, FontAssets}, board::Cell};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Reset>()
            .add_system(menu_button_system)
            .add_system(reset_events);
            
    }
}


pub struct Reset;

#[derive(EnumIter, Debug, Component)]
enum MenuButton {
    New,
    Reset,
    Quit,
}

impl Display for MenuButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuButton::New => write!(f, "New Game"),
            MenuButton::Reset => write!(f, "Reset"),
            MenuButton::Quit => write!(f, "Quit"),
        }
    }
}

pub fn create_menu(
    parent: &mut ChildBuilder,
    button_assets: &ButtonAssets,
    font_assets: &FontAssets,
) {
    // Menu buttons
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(20.0),                            
                    right: Val::Px(20.0),                            
                    ..default()
                },
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .insert(Name::new("Menu Buttons"))
        .with_children(|parent| {
            for button in MenuButton::iter() {
                let name = format!("{button}");
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                            margin: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        color: button_assets.normal.into(),
                        ..default()
                    })
                    .insert(Name::new(name.clone()))
                    .insert(button)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: name,
                                    style: TextStyle {
                                        font: font_assets.ui_font.clone(),
                                        font_size: 30.0,
                                        color: Color::BLACK,
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
    mut interaction_query: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,    
    mut reset_event: EventWriter<Reset>,    
    mut exit_event: EventWriter<AppExit>,    
) {
    for (interaction, menu_button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                info!("Clicked on menu button: {:?}", menu_button);
                match menu_button {
                    MenuButton::New => {
                        info!("New Game");
                    }
                    MenuButton::Reset => {
                        reset_event.send(Reset);
                    }
                    MenuButton::Quit => {
                        exit_event.send(AppExit);
                    }
                }
            }
            _ => {}
        }
    }
}

fn reset_events(
    mut reset_event: EventReader<Reset>,
    mut query: Query<&mut Cell>,
) {
    for _ in reset_event.iter() {
        for mut cell in query.iter_mut() {
            cell.reset();
        }
    }
}
