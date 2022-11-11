use bevy::prelude::*;

use crate::ui::{ButtonAssets, FontAssets};
use std::fmt::{Display, Formatter};
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoardSelect>()
            .add_event::<CellSelect>()
            .init_resource::<BoardSelected>()
            .init_resource::<CellMenuKeys>()
            .add_system(board_cell_button_system)
            .add_system(cell_menu_button_system)            
            .add_system(cell_select_event)
            .add_system(cell_menu_select_event)
            .add_system(update_cell_text)
            .add_system(cell_keyboard_input);
    }
}

/// Event for when a board button is selected
pub struct BoardSelect(pub Entity);

/// Event for when a cell menu button is selected
pub struct CellSelect(pub Value);

/// Resource for currently selected board entity
#[derive(Default)]
pub struct BoardSelected {
    pub entity: Option<Entity>,
}
    

pub fn create_board(
    parent: &mut ChildBuilder,
    button_assets: &ButtonAssets,
    font_assets: &FontAssets,
) {
    // board
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                border: UiRect::all(Val::Px(2.0)),
                align_content: AlignContent::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: Color::RED.into(),
            ..default()
        })
        .insert(Name::new("Board"))
        .with_children(|parent| {
            // rows
            for x in 0..9 {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        color: Color::BLUE.into(),
                        ..default()
                    })
                    .insert(Name::new(format!("Row {}", x)))
                    .with_children(|parent| {
                        // columns
                        for y in 0..9 {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                        margin: UiRect::all(Val::Px(2.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    color: button_assets.normal.into(),
                                    ..default()
                                })
                                .insert(Name::new(format!("Cell {x}x{y}")))
                                .insert(Cell::default())
                                .insert(CellPosition::new(x, y))
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: "".to_string(),
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
        });
}

#[derive(Component)]
pub struct CellMenuButton(pub Value);

pub fn create_board_buttons(parent: &mut ChildBuilder, button_assets: &ButtonAssets, font_assets: &FontAssets) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(2.0)),
                margin: UiRect {
                    top: Val::Px(20.0),
                    ..default()
                },
                ..default()
            },
            color: Color::RED.into(),
            ..default()
        })
        .insert(Name::new("Buttons"))
        .with_children(|parent| {
            for i in 1..=9 {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                            // center button
                            margin: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        color: button_assets.normal.into(),
                        ..default()
                    })
                    .insert(Name::new(format!("Option {i}")))
                    .insert(CellMenuButton(Value::from(i)))
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: format!("{i}"),
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

fn board_cell_button_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiColor, &Cell),
        (Changed<Interaction>, With<Button>),
    >,
    button_assets: Res<ButtonAssets>,
    mut select_event: EventWriter<BoardSelect>,
    selected: Res<BoardSelected>,
) {
    for (e, interaction, mut color, cell) in &mut interaction_query {
        if cell.is_disabled() {
            continue;
        }
        match *interaction {
            Interaction::Clicked => {
                *color = button_assets.selected.into();
                select_event.send(BoardSelect(e));
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                *color = button_assets.hovered.into();
            }
            Interaction::None => {
                if selected.entity.is_some() && e == selected.entity.unwrap() {
                    *color = button_assets.selected.into();
                } else {
                    *color = button_assets.normal.into();
                }
            }
        }
    }
}

fn cell_menu_button_system(
    mut select_event: EventWriter<CellSelect>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &CellMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    button_assets: Res<ButtonAssets>,    
) {
    
    for (interaction, mut color, cell_menu) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = button_assets.selected.into();
                select_event.send(CellSelect(cell_menu.0));
                info!("Clicked {}", cell_menu.0);
            }
            _ => {}
        }
    }
}

struct KeyValues {
    pub key: Vec<KeyCode>,
    pub value: Value,
}

struct CellMenuKeys(pub Vec<KeyValues>);

impl Default for CellMenuKeys {
    fn default() -> Self {
        Self( vec![
            KeyValues {
                key: vec![KeyCode::Key1, KeyCode::Numpad1],
                value: Value::One,
            },
            KeyValues {
                key: vec![KeyCode::Key2, KeyCode::Numpad2],
                value: Value::Two,
            },
            KeyValues {
                key: vec![KeyCode::Key3, KeyCode::Numpad3],
                value: Value::Three,
            },
            KeyValues {
                key: vec![KeyCode::Key4, KeyCode::Numpad4],
                value: Value::Four,
            },
            KeyValues {
                key: vec![KeyCode::Key5, KeyCode::Numpad5],
                value: Value::Five,
            },
            KeyValues {
                key: vec![KeyCode::Key6, KeyCode::Numpad6],
                value: Value::Six,
            },
            KeyValues {
                key: vec![KeyCode::Key7, KeyCode::Numpad7],
                value: Value::Seven,
            },
            KeyValues {
                key: vec![KeyCode::Key8, KeyCode::Numpad8],
                value: Value::Eight,
            },
            KeyValues {
                key: vec![KeyCode::Key9, KeyCode::Numpad9],
                value: Value::Nine,
            },
        ])
     
    }
}


fn cell_keyboard_input(

    mut select_event: EventWriter<CellSelect>,
    selected: Res<BoardSelected>,
    keyboard_input: Res<Input<KeyCode>>,
    cell_menu_keys: Res<CellMenuKeys>,
) {
    if selected.entity.is_none() {
        return;
    }

    for key_value in cell_menu_keys.0.iter() {
        for key in key_value.key.iter() {
            if keyboard_input.just_pressed(*key) {
                select_event.send(CellSelect(key_value.value));
            }
        }
        
    }
}

fn cell_select_event(
    mut select_events: EventReader<BoardSelect>,
    mut board_selected: ResMut<BoardSelected>,
    mut query: Query<(&mut UiColor, &Cell)>,
    button_assets: Res<ButtonAssets>,
) {
    for event in select_events.iter() {
        
        // clear previous selection
        if let Some(selected_entity) = board_selected.entity {
            if let Ok((mut color, _)) = query.get_mut(selected_entity) {
                if selected_entity != event.0 { 
                    *color = button_assets.normal.into();
                }
            }            
        }
        
        // set new selection
        board_selected.entity = Some(event.0);            
        
    }
}

fn cell_menu_select_event(
    mut select_events: EventReader<CellSelect>,
    board_selected: Res<BoardSelected>,
    mut query: Query<&mut Cell>,    
) {
    for event in select_events.iter() {        
        // clear previous selection
        if let Some(selected_entity) = board_selected.entity {
            if let Ok(mut cell) = query.get_mut(selected_entity) {
                cell.value = Some(event.0);      
      
            }            
        }
    }
}



fn update_cell_text(
    query: Query<(&Cell, &Children), Changed<Cell>>,
    mut text_query: Query<&mut Text>,
) {
    for (cell, children) in &mut query.iter() {
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(*child) {
                text.sections[0].value = match cell.value {
                    Some(v) => format!("{v}"),
                    None => "".to_string(),
                };
            }
        }
    }
}

#[derive(Component)]
pub struct CellPosition {
    x: usize,
    y: usize,
}

impl CellPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
pub struct Cell {
    value: Option<Value>,
    enabled: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: None,
            enabled: true,
        }
    }
}

impl Cell {
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    #[allow(dead_code)]
    pub fn is_disabled(&self) -> bool {
        !self.enabled
    }

    
    pub fn reset(&mut self) {
        if self.enabled {
            self.value = None;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        match value {
            1 => Value::One,
            2 => Value::Two,
            3 => Value::Three,
            4 => Value::Four,
            5 => Value::Five,
            6 => Value::Six,
            7 => Value::Seven,
            8 => Value::Eight,
            9 => Value::Nine,
            _ => panic!("Invalid value"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::One => write!(f, "1"),
            Value::Two => write!(f, "2"),
            Value::Three => write!(f, "3"),
            Value::Four => write!(f, "4"),
            Value::Five => write!(f, "5"),
            Value::Six => write!(f, "6"),
            Value::Seven => write!(f, "7"),
            Value::Eight => write!(f, "8"),
            Value::Nine => write!(f, "9"),
        }
    }
}