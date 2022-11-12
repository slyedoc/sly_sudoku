use bevy::prelude::*;

use crate::ui::*;
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
            .add_system(cell_keyboard_input)
            .add_system(update_cell_text);
    }
}

/// Event for when a board button is selected
pub struct BoardSelect(pub Entity);

/// Event for when a cell menu button is selected
pub struct CellSelect(pub Option<Value>);

/// Resource for currently selected board entity
#[derive(Default)]
pub struct BoardSelected {
    pub entity: Option<Entity>,
}

pub fn create_board(
    parent: &mut ChildBuilder,
    theme: &Theme,
    font_assets: &FontAssets,
) {
    // board
    let bold_line_thickness = 4.0;
    let bold_line = Val::Px(bold_line_thickness);
    let half_bold_line = Val::Px(bold_line_thickness * 0.5);
    let thin_line_thickness = 2.0;
    let half_thin_line = Val::Px(thin_line_thickness * 0.5);

    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                align_content: AlignContent::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                padding: UiRect::all(Val::Px(0.0)),
                ..default()
            },
            color: theme.line_thick.into(),
            ..default()
        })
        .insert(ThickLine)        
        .insert(Name::new("Board"))
        .with_children(|parent| {
            // since I want the grid lines to look right, I need to create the grid
            // as a 3v3 grid of 3x3 cells
            for grid_y in 0..3 {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            ..default()
                        },
                        color: Color::NONE.into(),
                        ..default()
                    })
                    .insert(Name::new(format!("Grid Row {}", grid_y)))
                    .with_children(|parent| {
                        for grid_x in 0..3 {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::ColumnReverse,
                                        justify_content: JustifyContent::Center,                                        
                                        margin: UiRect {
                                            left: if grid_x != 0  {
                                                half_bold_line
                                            } else {
                                                bold_line
                                            },                                                                    
                                            right: if grid_x != 2 {
                                                half_bold_line
                                            } else {
                                                bold_line
                                            },
                                            top: if grid_y != 0  {
                                                half_bold_line
                                            } else {
                                                bold_line
                                            }, 
                                            bottom: if grid_y != 2  {
                                                half_bold_line
                                            } else {
                                                bold_line
                                            },
                                        },
                                        ..default()
                                    },
                                    color: theme.line_thin.into(),
                                    ..default()
                                })
                                .insert(ThinLine)
                                .insert(Name::new(format!("Grid {grid_x}x{grid_y}")))
                                .with_children(|parent| {
                                    // 3v3 cells
                                    for y in 0..3 {
                                        let pos_y = grid_y * 3 + y;
                                        parent
                                            .spawn_bundle(NodeBundle {
                                                style: Style {
                                                    flex_direction: FlexDirection::Row,
                                                    justify_content: JustifyContent::Center,
                                                    margin: UiRect::all(Val::Px(0.0)),
                                                    ..default()
                                                },
                                                color: Color::NONE.into(),
                                                ..default()
                                            })
                                            .insert(Name::new(format!("Row {}", y)))
                                            .with_children(|parent| {
                                                // columns
                                                for x in 0..3 {
                                                    let pos_x = grid_x * 3 + x;
                                                    parent
                                                        .spawn_bundle(ButtonBundle {
                                                            style: Style {
                                                                size: Size::new(
                                                                    Val::Px(50.0),
                                                                    Val::Px(50.0),
                                                                ),
                                                                margin: UiRect {
                                                                    left: if x != 0  {
                                                                        half_thin_line
                                                                    } else {
                                                                        Val::Px(0.0)
                                                                    },                                                                    
                                                                    right: if x != 2 {
                                                                        half_thin_line
                                                                    } else {
                                                                        Val::Px(0.0)
                                                                    },
                                                                    top: if y != 0  {
                                                                        half_thin_line
                                                                    } else {
                                                                        Val::Px(0.0)
                                                                    }, 
                                                                    bottom: if y != 2  {
                                                                        half_thin_line
                                                                    } else {
                                                                        Val::Px(0.0)
                                                                    }, 
                                                                    ..default()
                                                                },
                                                                justify_content: JustifyContent::Center,
                                                                align_items: AlignItems::Center,
                                                                ..default()
                                                            },
                                                            color: theme.btn_normal.into(),
                                                            ..default()
                                                        })
                                                        .insert(Name::new(format!(
                                                            "Cell {pos_x}x{pos_y}"
                                                        )))
                                                        .insert(Cell::default())
                                                        .insert(CellPosition::new(pos_x, pos_y))
                                                        .with_children(|parent| {
                                                            parent.spawn_bundle(TextBundle {
                                                                text: Text {
                                                                    sections: vec![TextSection {
                                                                        value: format!(
                                                                            "({pos_x},{pos_y})"
                                                                        ),
                                                                        //value: "".to_string(),
                                                                        style: TextStyle {
                                                                            font: font_assets
                                                                                .ui_font
                                                                                .clone(),
                                                                            font_size: 30.0,
                                                                            color: Color::BLACK,
                                                                        },
                                                                    }],
                                                                    alignment: TextAlignment {
                                                                        vertical:
                                                                            VerticalAlign::Center,
                                                                        horizontal:
                                                                            HorizontalAlign::Center,
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
                    });
            }
        });
}

#[derive(Component)]
pub struct CellMenuButton(pub Option<Value>);

pub fn create_board_buttons(
    parent: &mut ChildBuilder,
    theme: &Theme,
    font_assets: &FontAssets,
) {
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
            color: theme.line_thick.into(),
            ..default()
        })
        .insert(ThickLine)
        .insert(Name::new("Buttons"))
        .with_children(|parent| {
            for i in 0..=9 {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                            margin: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        color: theme.btn_normal.into(),
                        ..default()
                    })
                    .insert(Name::new(format!("Option {i}")))
                    .insert( if i > 0 {
                        CellMenuButton(Some(Value::from(i)))
                    } else {
                        CellMenuButton(None)
                    })                    
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: if i > 0 {
                                        format!("{i}")
                                    } else {
                                        "".to_string()
                                    },
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
    theme: Res<ThemeMode>,
    mut select_event: EventWriter<BoardSelect>,
    selected: Res<BoardSelected>,
) {
    for (e, interaction, mut color, cell) in &mut interaction_query {
        let theme = theme.theme();
        if cell.is_disabled() {
            continue;
        }
        match *interaction {
            Interaction::Clicked => {
                *color = theme.btn_selected.into();
                select_event.send(BoardSelect(e));
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                *color = theme.btn_hovered.into();
            }
            Interaction::None => {
                if selected.entity.is_some() && e == selected.entity.unwrap() {
                    *color = theme.btn_selected.into();
                } else {
                    *color = theme.btn_normal.into();
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
    theme: Res<ThemeMode>,
) {
    
    for (interaction, mut color, cell_menu) in &mut interaction_query {
        let theme = theme.theme();
        match *interaction {
            Interaction::Clicked => {
                *color = theme.btn_selected.into();
                select_event.send(CellSelect(cell_menu.0));
            }
            _ => {}
        }
    }
}

struct KeyValues {
    pub key: Vec<KeyCode>,
    pub value: Option<Value>,
}

struct CellMenuKeys(pub Vec<KeyValues>);

impl Default for CellMenuKeys {
    fn default() -> Self {
        Self(vec![
            KeyValues {
                key: vec![KeyCode::Key1, KeyCode::Numpad1],
                value: Some(Value::One),
            },
            KeyValues {
                key: vec![KeyCode::Key2, KeyCode::Numpad2],
                value: Some(Value::Two),
            },
            KeyValues {
                key: vec![KeyCode::Key3, KeyCode::Numpad3],
                value: Some(Value::Three),
            },
            KeyValues {
                key: vec![KeyCode::Key4, KeyCode::Numpad4],
                value: Some(Value::Four),
            },
            KeyValues {
                key: vec![KeyCode::Key5, KeyCode::Numpad5],
                value: Some(Value::Five),
            },
            KeyValues {
                key: vec![KeyCode::Key6, KeyCode::Numpad6],
                value: Some(Value::Six),
            },
            KeyValues {
                key: vec![KeyCode::Key7, KeyCode::Numpad7],
                value: Some(Value::Seven),
            },
            KeyValues {
                key: vec![KeyCode::Key8, KeyCode::Numpad8],
                value: Some(Value::Eight),
            },
            KeyValues {
                key: vec![KeyCode::Key9, KeyCode::Numpad9],
                value: Some(Value::Nine),
            },
            KeyValues {
                key: vec![KeyCode::Delete, KeyCode::Back],
                value: None,
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
    button_assets: Res<Theme>,
) {
    for event in select_events.iter() {
        // clear previous selection
        if let Some(selected_entity) = board_selected.entity {
            if let Ok((mut color, _)) = query.get_mut(selected_entity) {
                if selected_entity != event.0 {
                    *color = button_assets.btn_normal.into();
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
        // set new selection
        if let Some(selected_entity) = board_selected.entity {
            if let Ok(mut cell) = query.get_mut(selected_entity) {
                cell.value = event.0;
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
    pub x: usize,
    pub y: usize,
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
    pub fn value(&self) -> Option<Value> {
        self.value
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = Some(value);
    }

    pub fn init_value(&mut self, value: Option<usize>) {
        match value {
            Some(v) => {
                self.value = Some(Value::from(v));
                self.enabled = false;
            }
            None => {
                self.value = None;
                self.enabled = true;
            }
        };
    }

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

impl From<Value> for usize {
    fn from(value: Value) -> Self {
        match value {
            Value::One => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
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
